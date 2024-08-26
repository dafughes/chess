import init, {
  Board,
  Color,
  File,
  Move,
  Piece,
  PieceKind,
  Rank,
  search,
  SearchParams,
  Square,
} from "./pkg/chess.js";

const GameResult = {
  WhiteWins: 0,
  BlackWins: 1,
  Draw: 2,
  Ongoing: 3,
};

function pieceToUnicode(piece) {
  switch (piece) {
    case Piece.WhitePawn:
      return "\u265F";
    case Piece.WhiteKnight:
      return "\u265E";
    case Piece.WhiteBishop:
      return "\u265D";
    case Piece.WhiteRook:
      return "\u265C";
    case Piece.WhiteQueen:
      return "\u265B";
    case Piece.WhiteKing:
      return "\u265A";
    case Piece.BlackPawn:
      return "\u265F";
    case Piece.BlackKnight:
      return "\u265E";
    case Piece.BlackBishop:
      return "\u265D";
    case Piece.BlackRook:
      return "\u265C";
    case Piece.BlackQueen:
      return "\u265B";
    case Piece.BlackKing:
      return "\u265A";
    default:
      return null;
  }
}

function drawBoard(squareSize, lightColor, darkColor, whitePov = true) {
  const canvas = document.getElementById("board");
  const ctx = canvas.getContext("2d");

  for (let y = 0; y < 8; y++) {
    for (let x = 0; x < 8; x++) {
      if (y % 2 == x % 2) {
        ctx.fillStyle = lightColor;
      } else {
        ctx.fillStyle = darkColor;
      }

      ctx.fillRect(
        x * squareSize,
        y * squareSize,
        squareSize,
        squareSize,
      );
    }
  }
}

function drawPiece(piece, squareSize, x, y) {
  const symbol = pieceToUnicode(piece);
  if (symbol) {
    const canvas = document.getElementById("board");
    const ctx = canvas.getContext("2d");
    ctx.textAlign = "center";

    ctx.shadowColor = (piece < 6)
      ? "rgba(0 0 0 / 70%)"
      : "rgba(200 200 200 / 70%)";
    ctx.shadowBlur = 7;

    ctx.font = squareSize.toString() * 1.2 + "px serif";
    ctx.fillStyle = (piece < 6) ? "rgb(255 255 210)" : "rgb(0 0 10)";
    ctx.fillText(symbol, x + squareSize / 2, y + squareSize * 0.95);
  }
}

function drawHighlight(color, squareSize, x, y) {
  const canvas = document.getElementById("board");
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.arc(
    x + squareSize / 2,
    y + squareSize / 2,
    squareSize / 3,
    0,
    2 * Math.PI,
  );
  ctx.fill();
}

function getSquareFromCoordinates(x, y, squareSize, whitePov = true) {
  const rank = whitePov
    ? 7 - Math.floor(y / squareSize)
    : Math.floor(y / squareSize);
  const file = whitePov
    ? Math.floor(x / squareSize)
    : 7 - Math.floor(x / squareSize);
  return Square.from_index(rank * 8 + file);
}

function getSquareXY(square, squareSize, whitePov = true) {
  const x = whitePov
    ? square.file() * squareSize
    : (7 - square.file()) * squareSize;
  const y = whitePov
    ? (7 - square.rank()) * squareSize
    : square.rank() * squareSize;

  return [x, y];
}

function moveToString(move) {
  const squareToString = (square) => {
    const fileChar = String.fromCharCode(square.file() + "a".charCodeAt(0));
    const rankChar = String.fromCharCode(square.rank() + "1".charCodeAt(0));
    return fileChar + rankChar;
  };

  let result = squareToString(move.from()) + squareToString(move.to());

  switch (move.promotion_kind()) {
    case PieceKind.Queen:
      result += "q";
      break;
    case PieceKind.Rook:
      result += "r";
      break;
    case PieceKind.Bishop:
      result += "b";
      break;
    case PieceKind.Knight:
      result += "n";
      break;
    default:
      break;
  }

  return result;
}

class Game {
  constructor(playerColor, width) {
    this.player = playerColor;
    this.whitePov = this.player === Color.White;
    this.width = width;
    this.squareSize = Math.floor(width / 8);

    // Resize canvas
    const canvas = document.getElementById("board");
    canvas.width = this.width;
    canvas.height = this.width;

    // State flags
    this.gameOver = false;
    this.fromSquare = null;
    this.toSquare = null;
    this.hasPromSelector = false;
    this.promotionHoverX = null;
    this.promotionHoverY = null;
    this.promSelX = null;
    this.promSelY = null;

    this.opponentLastMoveSquares = [];
    this.higlightSquares = [];

    // this.board = Board.default();
    this.board = Board.from_fen("8/3P4/8/k7/8/K7/8/8 w - -");

    this.lightColor = "rgb(219, 232, 200)";
    this.darkColor = "rgb(72, 79, 61)";
    this.highlightColor = "rgb(255 200 0 / 30%)";
    this.opponentMoveColor = "rgb(255 50 0 / 30%)";
    this.promotionSelectorColor = "rgba(219, 232, 200, 50%)";
  }

  engineTurn() {
    if (this.gameOver) {
      return;
    }

    const params = new SearchParams();

    params.depth = 2;

    const move = search(params, this.board);

    this.opponentLastMoveSquares = [move.from(), move.to()];

    this.makeMove(move);
  }

  makeMove(move) {
    this.board = this.board.do_move(move);
    this.draw();

    switch (this.result()) {
      case GameResult.WhiteWins:
        console.log("White wins!");
        this.gameOver = true;
        break;
      case GameResult.BlackWins:
        console.log("Black wins!");
        this.gameOver = true;
        break;
      case GameResult.Draw:
        console.log("Draw");
        this.gameOver = true;
        break;
      default:
        break;
    }
  }

  update(x, y) {
    if (this.gameOver) {
      return;
    }

    const square = getSquareFromCoordinates(
      x,
      y,
      this.squareSize,
      this.whitePov,
    );

    if (this.fromSquare) {
      // Check if promotion selection
      if (this.toSquare) {
        const piece = this.getPromotionPiece(x, y);

        if (piece) {
          const move = this.board.legal_moves().find((move) =>
            (move.from().to_index() === this.fromSquare.to_index()) &&
            (move.to().to_index() === this.toSquare.to_index()) &&
            (move.promotion_kind() === piece)
          );

          if (move) {
            this.fromSquare = null;
            this.toSquare = null;

            this.hasPromSelector = false;
            this.promSelX = null;
            this.promSelY = null;

            this.makeMove(move);

            this.engineTurn();
          }
        }

        this.fromSquare = null;
        this.toSquare = null;

        this.hasPromSelector = false;
        this.promSelX = null;
        this.promSelY = null;
      } else {
        // Check if move from selectedSquare to square is valid.
        const moves = this.board.legal_moves().filter((move) =>
          (move.from().to_index() === this.fromSquare.to_index()) &&
          (move.to().to_index() === square.to_index())
        );

        if (moves.length > 1) {
          this.toSquare = square;

          this.hasPromSelector = true;
          const [xx, yy] = getSquareXY(square, this.squareSize, this.whitePov);
          this.promSelX = xx;
          this.promSelY = yy;
        } else if (moves.length === 1) {
          let move = moves[0];

          this.makeMove(move);

          this.fromSquare = null;
          this.toSquare = null;

          this.engineTurn();
        } else {
          this.fromSquare = null;
          this.toSquare = null;
          this.higlightSquares = [];
        }
        this.higlightSquares = [];
        this.draw();
      }
    } else {
      // Get legal moves from clicked square.
      const moves = this.board.legal_moves().filter((move) =>
        move.from().to_index() === square.to_index()
      );

      if (moves.length > 0) {
        this.fromSquare = square;
        this.higlightSquares = moves.map((move) => move.to());
      }
      this.draw();
    }
  }

  draw() {
    // board
    drawBoard(this.squareSize, this.lightColor, this.darkColor, this.whitePov);

    // pieces
    for (let i = 0; i < 64; i++) {
      const square = Square.from_index(i);
      const piece = this.board.at(square);
      const [x, y] = getSquareXY(square, this.squareSize, this.whitePov);

      drawPiece(piece, this.squareSize, x, y);
    }

    // possible highlighed squares
    for (const square of this.higlightSquares) {
      const [x, y] = getSquareXY(square, this.squareSize, this.whitePov);

      drawHighlight(this.highlightColor, this.squareSize, x, y);
    }

    // Opponents last move
    for (const square of this.opponentLastMoveSquares) {
      const [x, y] = getSquareXY(square, this.squareSize, this.whitePov);

      drawHighlight(this.opponentMoveColor, this.squareSize, x, y);
    }

    // Possible promotion selector
    if (this.hasPromSelector) {
      this.drawPromotionSelector(this.promSelX, this.promSelY);
    }
  }

  updatePromotionSelector(x, y) {
    this.promotionHoverX = x;
    this.promotionHoverY = y;

    this.draw();
  }

  getPromotionPiece(x, y) {
    const pieces = this.player === Color.White
      ? [
        Piece.WhiteQueen,
        Piece.WhiteRook,
        Piece.WhiteBishop,
        Piece.WhiteKnight,
      ]
      : [
        Piece.BlackQueen,
        Piece.BlackRook,
        Piece.BlackBishop,
        Piece.BlackKnight,
      ];

    const offset = this.squareSize * 0.1;

    for (let i = 0; i < pieces.length; i++) {
      const left = this.promSelX + this.squareSize + offset;
      const right = left + this.squareSize;
      const top = this.promSelY + offset + i * this.squareSize;
      const bottom = top + this.squareSize;

      // Is hovered?
      if (
        x >= left && x < right &&
        y >= top && y < bottom
      ) {
        return pieces[i];
      }
    }

    return null;
  }

  drawPromotionSelector(x, y) {
    const canvas = document.getElementById("board");
    const ctx = canvas.getContext("2d");

    const pieces = this.player === Color.White
      ? [
        Piece.WhiteQueen,
        Piece.WhiteRook,
        Piece.WhiteBishop,
        Piece.WhiteKnight,
      ]
      : [
        Piece.BlackQueen,
        Piece.BlackRook,
        Piece.BlackBishop,
        Piece.BlackKnight,
      ];

    const offset = this.squareSize * 0.1;

    for (let i = 0; i < pieces.length; i++) {
      const left = x + this.squareSize + offset;
      const right = left + this.squareSize;
      const top = y + offset + i * this.squareSize;
      const bottom = top + this.squareSize;

      // Is hovered?
      if (
        this.promotionHoverX && this.promotionHoverY &&
        this.promotionHoverX >= left && this.promotionHoverX < right &&
        this.promotionHoverY >= top && this.promotionHoverY < bottom
      ) {
        ctx.fillStyle = "rgba(230, 240, 210, 50%)";
      } else {
        ctx.fillStyle = "rgba(200, 210, 180, 50%)";
      }

      ctx.fillRect(
        left,
        y + offset + i * this.squareSize,
        this.squareSize,
        this.squareSize,
      );

      drawPiece(
        pieces[i],
        this.squareSize,
        left,
        top,
      );
    }
  }

  // TODO: 50 move rule, repetitions etc
  result() {
    const moves = this.board.legal_moves();

    if (moves.length === 0) {
      if (this.board.is_in_check()) {
        return this.board.color_to_move() === Color.White
          ? GameResult.BlackWins
          : GameResult.WhiteWins;
      }
      return GameResult.Draw;
    }

    return GameResult.Ongoing;
  }
}

init().then(() => {
  let game = new Game(Color.White, 800);
  game.draw();

  if (game.player === Color.Black) {
    game.engineTurn();
  }

  const canvas = document.getElementById("board");
  canvas.addEventListener("click", (e) => {
    var rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    game.update(x, y);
  });

  canvas.addEventListener("mousemove", (e) => {
    var rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (game.hasPromSelector) {
      game.updatePromotionSelector(x, y);
    }
  });
});
