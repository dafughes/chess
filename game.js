import init, { Game, Move } from "./pkg/chess.js";

// Piece constants
const Piece = {
  WhitePawn: 0,
  WhiteKnight: 1,
  WhiteBishop: 2,
  WhiteRook: 3,
  WhiteQueen: 4,
  WhiteKing: 5,
  BlackPawn: 8,
  BlackKnight: 9,
  BlackBishop: 10,
  BlackRook: 11,
  BlackQueen: 12,
  BlackKing: 13,
  None: 16,
};

class Board {
  constructor(squareSize, whitePov) {
    this.squareSize = squareSize;

    this.width = 8 * squareSize;
    this.height = 8 * squareSize;

    // Resize canvas
    const canvas = document.getElementById("board");
    canvas.width = this.width;
    canvas.height = this.height;

    this.whitePov = whitePov;

    this.lightColor = "rgb(173 204 162)";
    this.darkColor = "rgb(57 58 53)";

    // Piece symbols
    this.symbolMap = new Map();
    this.symbolMap.set(Piece.WhitePawn, "\u265F");
    this.symbolMap.set(Piece.WhiteKnight, "\u265E");
    this.symbolMap.set(Piece.WhiteBishop, "\u265D");
    this.symbolMap.set(Piece.WhiteRook, "\u265C");
    this.symbolMap.set(Piece.WhiteQueen, "\u265B");
    this.symbolMap.set(Piece.WhiteKing, "\u265A");
    this.symbolMap.set(Piece.BlackPawn, "\u265F");
    this.symbolMap.set(Piece.BlackKnight, "\u265E");
    this.symbolMap.set(Piece.BlackBishop, "\u265D");
    this.symbolMap.set(Piece.BlackRook, "\u265C");
    this.symbolMap.set(Piece.BlackQueen, "\u265B");
    this.symbolMap.set(Piece.BlackKing, "\u265A");
  }

  draw(game) {
    this.drawGrid();
    for (let r = 0; r < 8; r++) {
      for (let f = 0; f < 8; f++) {
        let square = new Square(r, f);
        this.drawPiece(game.piece_at(square.toIndex()), square);
      }
    }
  }

  drawGrid() {
    const canvas = document.getElementById("board");
    const ctx = canvas.getContext("2d");

    for (let y = 0; y < 8; y++) {
      for (let x = 0; x < 8; x++) {
        if (y % 2 == x % 2) {
          ctx.fillStyle = this.lightColor;
        } else {
          ctx.fillStyle = this.darkColor;
        }

        ctx.fillRect(
          x * this.squareSize,
          y * this.squareSize,
          this.squareSize,
          this.squareSize,
        );
      }
    }
  }

  drawPiece(piece, square) {
    if (this.symbolMap.has(piece)) {
      const canvas = document.getElementById("board");
      const ctx = canvas.getContext("2d");
      ctx.textAlign = "center";

      const [x, y] = this.getSquareXY(square);

      var symbol = this.symbolMap.get(piece);

      ctx.font = this.squareSize.toString() * 1.2 + "px serif";
      ctx.fillStyle = (piece & 8) === 0 ? "rgb(255 255 255)" : "rgb(0 0 0)";
      ctx.fillText(symbol, x + this.squareSize / 2, y + this.squareSize * 0.95);
    }
  }

  drawHighlight(square) {
    const canvas = document.getElementById("board");
    const ctx = canvas.getContext("2d");
    const [x, y] = this.getSquareXY(square);
    ctx.fillStyle = "rgb(255  0 0 / 30%)";
    ctx.beginPath();
    ctx.arc(
      x + this.squareSize / 2,
      y + this.squareSize / 2,
      this.squareSize / 3,
      0,
      2 * Math.PI,
    );
    ctx.fill();
  }

  getSquareFromCoordinates(x, y) {
    if (this.whitePov) {
      return new Square(
        7 - Math.floor(y / this.squareSize),
        Math.floor(x / this.squareSize),
      );
    } else {
      return new Square(
        Math.floor(y / this.squareSize),
        7 - Math.floor(x / this.squareSize),
      );
    }
  }

  getSquareXY(square) {
    if (this.whitePov) {
      const x = square.file * this.squareSize;
      const y = (7 - square.rank) * this.squareSize;
      return [x, y];
    } else {
      const x = (7 - square.file) * this.squareSize;
      const y = square.rank * this.squareSize;
      return [x, y];
    }
  }
}

class Square {
  constructor(rank, file) {
    this.rank = rank;
    this.file = file;
  }

  static fromIndex(index) {
    return new Square(Math.floor(index / 8), index % 8);
  }

  toIndex() {
    return this.rank * 8 + this.file;
  }
}

init().then(() => {
  let game = new Game();

  const board = new Board(100, true);
  const canvas = document.getElementById("board");

  var from;

  canvas.addEventListener("click", (e) => {
    var rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    board.draw(game);

    const square = board.getSquareFromCoordinates(x, y);

    if (from) {
      if (square.toIndex() !== from.toIndex()) {
        const to = square;

        const move = new Move(from.toIndex(), to.toIndex(), 5);
        game.make_move(move);

        board.draw(game);

        const opp_move = game.best_move(4);
        game.make_move(opp_move);

        board.draw(game);
        board.drawHighlight(Square.fromIndex(opp_move.from));
        board.drawHighlight(Square.fromIndex(opp_move.to));
      }
      from = null;
    } else {
      const moves = game.legal_moves().filter((move) =>
        move.from === square.toIndex()
      );

      if (moves.length !== 0) {
        from = square;
      }

      for (const move of moves) {
        board.drawHighlight(Square.fromIndex(move.to));
      }
    }
  });

  board.draw(game);
});
