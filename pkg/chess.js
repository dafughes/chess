let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @param {SearchParams} params
* @param {Board} board
* @returns {Move}
*/
export function search(params, board) {
    _assertClass(params, SearchParams);
    var ptr0 = params.__destroy_into_raw();
    _assertClass(board, Board);
    const ret = wasm.search(ptr0, board.__wbg_ptr);
    return Move.__wrap(ret);
}

/**
*/
export const PieceKind = Object.freeze({ Pawn:0,"0":"Pawn",Knight:1,"1":"Knight",Bishop:2,"2":"Bishop",Rook:3,"3":"Rook",Queen:4,"4":"Queen",King:5,"5":"King", });
/**
*/
export const File = Object.freeze({ A:0,"0":"A",B:1,"1":"B",C:2,"2":"C",D:3,"3":"D",E:4,"4":"E",F:5,"5":"F",G:6,"6":"G",H:7,"7":"H", });
/**
*/
export const Piece = Object.freeze({ WhitePawn:0,"0":"WhitePawn",WhiteKnight:1,"1":"WhiteKnight",WhiteBishop:2,"2":"WhiteBishop",WhiteRook:3,"3":"WhiteRook",WhiteQueen:4,"4":"WhiteQueen",WhiteKing:5,"5":"WhiteKing",BlackPawn:6,"6":"BlackPawn",BlackKnight:7,"7":"BlackKnight",BlackBishop:8,"8":"BlackBishop",BlackRook:9,"9":"BlackRook",BlackQueen:10,"10":"BlackQueen",BlackKing:11,"11":"BlackKing", });
/**
*/
export const Rank = Object.freeze({ First:0,"0":"First",Second:1,"1":"Second",Third:2,"2":"Third",Fourth:3,"3":"Fourth",Fifth:4,"4":"Fifth",Sixth:5,"5":"Sixth",Seventh:6,"6":"Seventh",Eighth:7,"7":"Eighth", });
/**
*/
export const Color = Object.freeze({ White:0,"0":"White",Black:1,"1":"Black", });
/**
*/
export const MoveKind = Object.freeze({ Quiet:0,"0":"Quiet",Cap:1,"1":"Cap",Double:2,"2":"Double",EnPassant:3,"3":"EnPassant",Castling:4,"4":"Castling",PromKnight:5,"5":"PromKnight",PromBishop:6,"6":"PromBishop",PromRook:7,"7":"PromRook",PromQueen:8,"8":"PromQueen",PromCapKnight:9,"9":"PromCapKnight",PromCapBishop:10,"10":"PromCapBishop",PromCapRook:11,"11":"PromCapRook",PromCapQueen:12,"12":"PromCapQueen", });

const BoardFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_board_free(ptr >>> 0, 1));
/**
*/
export class Board {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Board.prototype);
        obj.__wbg_ptr = ptr;
        BoardFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BoardFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr, 0);
    }
    /**
    * Creates an empty chessboard.
    * @returns {Board}
    */
    static new() {
        const ret = wasm.board_new();
        return Board.__wrap(ret);
    }
    /**
    * Creates a new chessboard with the standard starting position.
    * @returns {Board}
    */
    static default() {
        const ret = wasm.board_default();
        return Board.__wrap(ret);
    }
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
    static from_fen(fen) {
        const ptr0 = passStringToWasm0(fen, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.board_from_fen(ptr0, len0);
        return Board.__wrap(ret);
    }
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
    fen() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.board_fen(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
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
    at(square) {
        _assertClass(square, Square);
        const ret = wasm.board_at(this.__wbg_ptr, square.__wbg_ptr);
        return ret === 12 ? undefined : ret;
    }
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
    put(piece, square) {
        _assertClass(square, Square);
        var ptr0 = square.__destroy_into_raw();
        wasm.board_put(this.__wbg_ptr, piece, ptr0);
    }
    /**
    * Returns the player with the next move.
    * @returns {Color}
    */
    color_to_move() {
        const ret = wasm.board_color_to_move(this.__wbg_ptr);
        return ret;
    }
    /**
    * The current move number
    * @returns {number}
    */
    fullmove_number() {
        const ret = wasm.board_fullmove_number(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Number of half-turns since pawn move or capture.
    * @returns {number}
    */
    halfmove_clock() {
        const ret = wasm.board_halfmove_clock(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {Color} color
    * @returns {boolean}
    */
    can_castle_kingside(color) {
        const ret = wasm.board_can_castle_kingside(this.__wbg_ptr, color);
        return ret !== 0;
    }
    /**
    * @param {Color} color
    * @returns {boolean}
    */
    can_castle_queenside(color) {
        const ret = wasm.board_can_castle_queenside(this.__wbg_ptr, color);
        return ret !== 0;
    }
    /**
    * Returns en passant square, if last move was a pawn double push.
    * @returns {Square | undefined}
    */
    en_passant_square() {
        const ret = wasm.board_en_passant_square(this.__wbg_ptr);
        return ret === 0 ? undefined : Square.__wrap(ret);
    }
    /**
    * Sets the player with the next move.
    * @param {Color} color
    */
    set_color_to_move(color) {
        wasm.board_set_color_to_move(this.__wbg_ptr, color);
    }
    /**
    * Sets the current move number
    * @param {number} fullmove
    */
    set_fullmove_number(fullmove) {
        wasm.board_set_fullmove_number(this.__wbg_ptr, fullmove);
    }
    /**
    */
    increment_fullmove_number() {
        wasm.board_increment_fullmove_number(this.__wbg_ptr);
    }
    /**
    * Sets the number of half-turns since pawn move or capture.
    * @param {number} halfmove
    */
    set_halfmove_clock(halfmove) {
        wasm.board_set_halfmove_clock(this.__wbg_ptr, halfmove);
    }
    /**
    * @param {Color} color
    */
    add_castling_kingside(color) {
        wasm.board_add_castling_kingside(this.__wbg_ptr, color);
    }
    /**
    * @param {Color} color
    */
    add_castling_queenside(color) {
        wasm.board_add_castling_queenside(this.__wbg_ptr, color);
    }
    /**
    * @param {Square | undefined} [ep]
    */
    set_en_passant_square(ep) {
        let ptr0 = 0;
        if (!isLikeNone(ep)) {
            _assertClass(ep, Square);
            ptr0 = ep.__destroy_into_raw();
        }
        wasm.board_set_en_passant_square(this.__wbg_ptr, ptr0);
    }
    /**
    * Executes a move and updates the board state.
    * Returns an undo object which is used for unmaking the move.
    * @param {Move} mv
    * @returns {Board}
    */
    do_move(mv) {
        _assertClass(mv, Move);
        var ptr0 = mv.__destroy_into_raw();
        const ret = wasm.board_do_move(this.__wbg_ptr, ptr0);
        return Board.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    is_in_check() {
        const ret = wasm.board_is_in_check(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {(Move)[]}
    */
    legal_moves() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.board_legal_moves(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const MoveFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_move_free(ptr >>> 0, 1));
/**
*/
export class Move {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Move.prototype);
        obj.__wbg_ptr = ptr;
        MoveFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MoveFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_move_free(ptr, 0);
    }
    /**
    * @param {Square} from
    * @param {Square} to
    * @param {MoveKind} kind
    */
    constructor(from, to, kind) {
        _assertClass(from, Square);
        var ptr0 = from.__destroy_into_raw();
        _assertClass(to, Square);
        var ptr1 = to.__destroy_into_raw();
        const ret = wasm.move_new(ptr0, ptr1, kind);
        this.__wbg_ptr = ret >>> 0;
        MoveFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * @returns {Square}
    */
    from() {
        const ret = wasm.move_from(this.__wbg_ptr);
        return Square.__wrap(ret);
    }
    /**
    * @returns {Square}
    */
    to() {
        const ret = wasm.move_to(this.__wbg_ptr);
        return Square.__wrap(ret);
    }
    /**
    * @returns {MoveKind}
    */
    kind() {
        const ret = wasm.move_kind(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {PieceKind | undefined}
    */
    promotion_kind() {
        const ret = wasm.move_promotion_kind(this.__wbg_ptr);
        return ret === 6 ? undefined : ret;
    }
    /**
    * @returns {Move}
    */
    static null() {
        const ret = wasm.move_null();
        return Move.__wrap(ret);
    }
}

const SearchParamsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_searchparams_free(ptr >>> 0, 1));
/**
*/
export class SearchParams {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SearchParamsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_searchparams_free(ptr, 0);
    }
    /**
    * @returns {number}
    */
    get wtime() {
        const ret = wasm.__wbg_get_searchparams_wtime(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set wtime(arg0) {
        wasm.__wbg_set_searchparams_wtime(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get btime() {
        const ret = wasm.__wbg_get_searchparams_btime(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set btime(arg0) {
        wasm.__wbg_set_searchparams_btime(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get winc() {
        const ret = wasm.__wbg_get_searchparams_winc(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set winc(arg0) {
        wasm.__wbg_set_searchparams_winc(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get binc() {
        const ret = wasm.__wbg_get_searchparams_binc(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set binc(arg0) {
        wasm.__wbg_set_searchparams_binc(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get movestogo() {
        const ret = wasm.__wbg_get_searchparams_movestogo(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set movestogo(arg0) {
        wasm.__wbg_set_searchparams_movestogo(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get depth() {
        const ret = wasm.__wbg_get_searchparams_depth(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set depth(arg0) {
        wasm.__wbg_set_searchparams_depth(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get nodes() {
        const ret = wasm.__wbg_get_searchparams_nodes(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set nodes(arg0) {
        wasm.__wbg_set_searchparams_nodes(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get mate() {
        const ret = wasm.__wbg_get_searchparams_mate(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set mate(arg0) {
        wasm.__wbg_set_searchparams_mate(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get movetime() {
        const ret = wasm.__wbg_get_searchparams_movetime(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set movetime(arg0) {
        wasm.__wbg_set_searchparams_movetime(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get infinite() {
        const ret = wasm.__wbg_get_searchparams_infinite(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set infinite(arg0) {
        wasm.__wbg_set_searchparams_infinite(this.__wbg_ptr, arg0);
    }
    /**
    */
    constructor() {
        const ret = wasm.searchparams_new();
        this.__wbg_ptr = ret >>> 0;
        SearchParamsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const SquareFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_square_free(ptr >>> 0, 1));
/**
*/
export class Square {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Square.prototype);
        obj.__wbg_ptr = ptr;
        SquareFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SquareFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_square_free(ptr, 0);
    }
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
    constructor(rank, file) {
        const ret = wasm.square_new(rank, file);
        this.__wbg_ptr = ret >>> 0;
        SquareFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
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
    rank() {
        const ret = wasm.square_rank(this.__wbg_ptr);
        return ret;
    }
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
    file() {
        const ret = wasm.square_file(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} index
    * @returns {Square}
    */
    static from_index(index) {
        const ret = wasm.square_from_index(index);
        return Square.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    to_index() {
        const ret = wasm.square_to_index(this.__wbg_ptr);
        return ret >>> 0;
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_move_new = function(arg0) {
        const ret = Move.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_log_c018d1be03a5ab8d = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;



    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined' && Object.getPrototypeOf(module) === Object.prototype)
    ({module} = module)
    else
    console.warn('using deprecated parameters for `initSync()`; pass a single object instead')

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined' && Object.getPrototypeOf(module_or_path) === Object.prototype)
    ({module_or_path} = module_or_path)
    else
    console.warn('using deprecated parameters for the initialization function; pass a single object instead')

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('chess_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
