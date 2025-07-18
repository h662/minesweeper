/* tslint:disable */
/* eslint-disable */
export class Minesweeper {
  free(): void;
  constructor();
  update(): void;
  render(): void;
  set_mode(mode: string): void;
  current_mode(): string;
  restart(): void;
  is_game_over(): boolean;
  is_game_won(): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_minesweeper_free: (a: number, b: number) => void;
  readonly minesweeper_new: () => [number, number, number];
  readonly minesweeper_update: (a: number) => void;
  readonly minesweeper_render: (a: number) => void;
  readonly minesweeper_set_mode: (a: number, b: number, c: number) => void;
  readonly minesweeper_current_mode: (a: number) => [number, number];
  readonly minesweeper_restart: (a: number) => void;
  readonly minesweeper_is_game_over: (a: number) => number;
  readonly minesweeper_is_game_won: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly closure2_externref_shim: (a: number, b: number, c: any) => void;
  readonly __wbindgen_start: () => void;
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
