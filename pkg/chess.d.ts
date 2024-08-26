/* tslint:disable */
/* eslint-disable */
/**
* @param {SearchParams} params
* @param {Board} board
* @returns {Move}
*/
export function search(params: SearchParams, board: Board): Move;
/**
*/
export enum PieceKind {
  Pawn = 0,
  Knight = 1,
  Bishop = 2,
  Rook = 3,
  Queen = 4,
  King = 5,
}
/**
*/
export enum File {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
  F = 5,
  G = 6,
  H = 7,
}
/**
*/
export enum Piece {
  WhitePawn = 0,
  WhiteKnight = 1,
  WhiteBishop = 2,
  WhiteRook = 3,
  WhiteQueen = 4,
  WhiteKing = 5,
  BlackPawn = 6,
  BlackKnight = 7,
  BlackBishop = 8,
  BlackRook = 9,
  BlackQueen = 10,
  BlackKing = 11,
}
/**
*/
export enum Rank {
  First = 0,
  Second = 1,
  Third = 2,
  Fourth = 3,
  Fifth = 4,
  Sixth = 5,
  Seventh = 6,
  Eighth = 7,
}
/**
*/
export enum Color {
  White = 0,
  Black = 1,
}
/**
*/
export enum MoveKind {
  Quiet = 0,
  Cap = 1,
  Double = 2,
  EnPassant = 3,
  Castling = 4,
  PromKnight = 5,
  PromBishop = 6,
  PromRook = 7,
  PromQueen = 8,
  PromCapKnight = 9,
  PromCapBishop = 10,
  PromCapRook = 11,
  PromCapQueen = 12,
}
/**
*/
export class Board {
  free(): void;
/**
* Creates an empty chessboard.
* @returns {Board}
*/
  static new(): Board;
/**
* Creates a new chessboard with the standard starting position.
* @returns {Board}
*/
  static default(): Board;
/**
* Creates a new chessboard from FEN string.
* # Example
* ```
* # use chess::{board::Board};
* let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
* assert_eq!(board.fen(), "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
* let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
* assert_eq!(board.fen(), "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
* ```
* @param {string} fen
* @returns {Board}
*/
  static from_fen(fen: string): Board;
/**
* Returns the position in FEN.
* # Example
* ```
* # use chess::{board::Board};
* let board = Board::default();
* assert_eq!(board.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
* ```
* @returns {string}
*/
  fen(): string;
/**
* Returns the possible piece on `square`.
* # Example
* ```
* # use chess::{board::Board, square::Square, piece::Piece};
* assert_eq!(Board::default().at(Square::E4), None);
* assert_eq!(Board::default().at(Square::A1), Some(Piece::WhiteRook));
* assert_eq!(Board::default().at(Square::B7), Some(Piece::BlackPawn));
* ```
* @param {Square} square
* @returns {Piece | undefined}
*/
  at(square: Square): Piece | undefined;
/**
* Puts `piece` on `square`.
* # Example
* ```
* # use chess::{board::Board, square::Square, piece::Piece};
* let mut board = Board::new();
* assert_eq!(board.at(Square::F5), None);
* board.put(Piece::WhiteKnight, Square::F5);
* assert_eq!(board.at(Square::F5), Some(Piece::WhiteKnight));
*
* ```
* @param {Piece} piece
* @param {Square} square
*/
  put(piece: Piece, square: Square): void;
/**
* Returns the player with the next move.
* @returns {Color}
*/
  color_to_move(): Color;
/**
* The current move number
* @returns {number}
*/
  fullmove_number(): number;
/**
* Number of half-turns since pawn move or capture.
* @returns {number}
*/
  halfmove_clock(): number;
/**
* @param {Color} color
* @returns {boolean}
*/
  can_castle_kingside(color: Color): boolean;
/**
* @param {Color} color
* @returns {boolean}
*/
  can_castle_queenside(color: Color): boolean;
/**
* Returns en passant square, if last move was a pawn double push.
* @returns {Square | undefined}
*/
  en_passant_square(): Square | undefined;
/**
* Sets the player with the next move.
* @param {Color} color
*/
  set_color_to_move(color: Color): void;
/**
* Sets the current move number
* @param {number} fullmove
*/
  set_fullmove_number(fullmove: number): void;
/**
*/
  increment_fullmove_number(): void;
/**
* Sets the number of half-turns since pawn move or capture.
* @param {number} halfmove
*/
  set_halfmove_clock(halfmove: number): void;
/**
* @param {Color} color
*/
  add_castling_kingside(color: Color): void;
/**
* @param {Color} color
*/
  add_castling_queenside(color: Color): void;
/**
* @param {Square | undefined} [ep]
*/
  set_en_passant_square(ep?: Square): void;
/**
* Executes a move and updates the board state.
* Returns an undo object which is used for unmaking the move.
* @param {Move} mv
* @returns {Board}
*/
  do_move(mv: Move): Board;
/**
* @returns {boolean}
*/
  is_in_check(): boolean;
/**
* @returns {(Move)[]}
*/
  legal_moves(): (Move)[];
}
/**
*/
export class Move {
  free(): void;
/**
* @param {Square} from
* @param {Square} to
* @param {MoveKind} kind
*/
  constructor(from: Square, to: Square, kind: MoveKind);
/**
* @returns {Square}
*/
  from(): Square;
/**
* @returns {Square}
*/
  to(): Square;
/**
* @returns {MoveKind}
*/
  kind(): MoveKind;
/**
* @returns {PieceKind | undefined}
*/
  promotion_kind(): PieceKind | undefined;
/**
* @returns {Move}
*/
  static null(): Move;
}
/**
*/
export class SearchParams {
  free(): void;
/**
*/
  constructor();
/**
*/
  binc: number;
/**
*/
  btime: number;
/**
*/
  depth: number;
/**
*/
  infinite: boolean;
/**
*/
  mate: number;
/**
*/
  movestogo: number;
/**
*/
  movetime: number;
/**
*/
  nodes: number;
/**
*/
  winc: number;
/**
*/
  wtime: number;
}
/**
*/
export class Square {
  free(): void;
/**
* # Example
* ```
* # use chess::square::{Rank, File, Square};
* assert_eq!(Square::new(Rank::Second, File::E), Square::E2);
* assert_eq!(Square::new(Rank::Fifth, File::C), Square::C5);
* assert_eq!(Square::new(Rank::Sixth, File::B), Square::B6);
* assert_eq!(Square::new(Rank::Second, File::F), Square::F2);
* assert_eq!(Square::new(Rank::First, File::A), Square::A1);
* ```
* @param {Rank} rank
* @param {File} file
*/
  constructor(rank: Rank, file: File);
/**
* # Example
* ```
* # use chess::square::{Rank, File, Square};
* assert_eq!(Square::F8.rank(), Rank::Eighth);
* assert_eq!(Square::C4.rank(), Rank::Fourth);
* assert_eq!(Square::D6.rank(), Rank::Sixth);
* assert_eq!(Square::B7.rank(), Rank::Seventh);
* assert_eq!(Square::A3.rank(), Rank::Third);
* assert_eq!(Square::E4.rank(), Rank::Fourth);
* assert_eq!(Square::G8.rank(), Rank::Eighth);
* ```
* @returns {Rank}
*/
  rank(): Rank;
/**
* # Example
* ```
* # use chess::square::{Rank, File, Square};
* assert_eq!(Square::F8.file(), File::F);
* assert_eq!(Square::C4.file(), File::C);
* assert_eq!(Square::D6.file(), File::D);
* assert_eq!(Square::B7.file(), File::B);
* assert_eq!(Square::A3.file(), File::A);
* assert_eq!(Square::E4.file(), File::E);
* assert_eq!(Square::G8.file(), File::G);
* ```
* @returns {File}
*/
  file(): File;
/**
* @param {number} index
* @returns {Square}
*/
  static from_index(index: number): Square;
/**
* @returns {number}
*/
  to_index(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_board_free: (a: number, b: number) => void;
  readonly board_new: () => number;
  readonly board_default: () => number;
  readonly board_from_fen: (a: number, b: number) => number;
  readonly board_fen: (a: number, b: number) => void;
  readonly board_at: (a: number, b: number) => number;
  readonly board_put: (a: number, b: number, c: number) => void;
  readonly board_color_to_move: (a: number) => number;
  readonly board_fullmove_number: (a: number) => number;
  readonly board_halfmove_clock: (a: number) => number;
  readonly board_can_castle_kingside: (a: number, b: number) => number;
  readonly board_can_castle_queenside: (a: number, b: number) => number;
  readonly board_en_passant_square: (a: number) => number;
  readonly board_set_color_to_move: (a: number, b: number) => void;
  readonly board_set_fullmove_number: (a: number, b: number) => void;
  readonly board_increment_fullmove_number: (a: number) => void;
  readonly board_set_halfmove_clock: (a: number, b: number) => void;
  readonly board_add_castling_kingside: (a: number, b: number) => void;
  readonly board_add_castling_queenside: (a: number, b: number) => void;
  readonly board_set_en_passant_square: (a: number, b: number) => void;
  readonly board_do_move: (a: number, b: number) => number;
  readonly board_is_in_check: (a: number) => number;
  readonly board_legal_moves: (a: number, b: number) => void;
  readonly __wbg_move_free: (a: number, b: number) => void;
  readonly move_new: (a: number, b: number, c: number) => number;
  readonly move_from: (a: number) => number;
  readonly move_to: (a: number) => number;
  readonly move_kind: (a: number) => number;
  readonly move_promotion_kind: (a: number) => number;
  readonly move_null: () => number;
  readonly __wbg_searchparams_free: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_wtime: (a: number) => number;
  readonly __wbg_set_searchparams_wtime: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_btime: (a: number) => number;
  readonly __wbg_set_searchparams_btime: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_winc: (a: number) => number;
  readonly __wbg_set_searchparams_winc: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_binc: (a: number) => number;
  readonly __wbg_set_searchparams_binc: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_movestogo: (a: number) => number;
  readonly __wbg_set_searchparams_movestogo: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_depth: (a: number) => number;
  readonly __wbg_set_searchparams_depth: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_nodes: (a: number) => number;
  readonly __wbg_set_searchparams_nodes: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_mate: (a: number) => number;
  readonly __wbg_set_searchparams_mate: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_movetime: (a: number) => number;
  readonly __wbg_set_searchparams_movetime: (a: number, b: number) => void;
  readonly __wbg_get_searchparams_infinite: (a: number) => number;
  readonly __wbg_set_searchparams_infinite: (a: number, b: number) => void;
  readonly searchparams_new: () => number;
  readonly search: (a: number, b: number) => number;
  readonly square_new: (a: number, b: number) => number;
  readonly square_rank: (a: number) => number;
  readonly square_file: (a: number) => number;
  readonly square_from_index: (a: number) => number;
  readonly square_to_index: (a: number) => number;
  readonly __wbg_square_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
