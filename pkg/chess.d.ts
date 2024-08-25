/* tslint:disable */
/* eslint-disable */
/**
*/
export class Game {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} square
* @returns {number}
*/
  piece_at(square: number): number;
/**
* @param {Move} mv
*/
  make_move(mv: Move): void;
/**
* @returns {(Move)[]}
*/
  legal_moves(): (Move)[];
/**
* @param {number} depth
* @returns {Move}
*/
  best_move(depth: number): Move;
}
/**
*/
export class Move {
  free(): void;
/**
* @param {number} from
* @param {number} to
* @param {number} promotion_type
*/
  constructor(from: number, to: number, promotion_type: number);
/**
*/
  from: number;
/**
*/
  promotion_type: number;
/**
*/
  to: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_move_free: (a: number, b: number) => void;
  readonly __wbg_get_move_from: (a: number) => number;
  readonly __wbg_set_move_from: (a: number, b: number) => void;
  readonly __wbg_get_move_to: (a: number) => number;
  readonly __wbg_set_move_to: (a: number, b: number) => void;
  readonly __wbg_get_move_promotion_type: (a: number) => number;
  readonly __wbg_set_move_promotion_type: (a: number, b: number) => void;
  readonly move_new: (a: number, b: number, c: number) => number;
  readonly __wbg_game_free: (a: number, b: number) => void;
  readonly game_new: () => number;
  readonly game_piece_at: (a: number, b: number) => number;
  readonly game_make_move: (a: number, b: number) => void;
  readonly game_legal_moves: (a: number, b: number) => void;
  readonly game_best_move: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
