/* tslint:disable */
/* eslint-disable */
/**
* This is the entry point of the service worker.
* This function is responsible for loading a service worker script from the given URL.
* The implementation largely follows the JavaScript code above, but is written using wasm_bindgen
* @param {string} worker_url
* @param {boolean} _try_once
* @returns {Promise<Promise<any>>}
*/
export function register_service_worker(worker_url: string, _try_once: boolean): Promise<Promise<any>>;
/**
* A more simple version of the above function that doesn't try to handle all the cases
* This just calls `navigator.service_worker.register` and returns the promise
* @param {string} worker_url
* @returns {Promise<Promise<any>>}
*/
export function basic_register_service_worker(worker_url: string): Promise<Promise<any>>;
/**
*/
export function init_wasm(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly register_service_worker: (a: number, b: number, c: number) => number;
  readonly basic_register_service_worker: (a: number, b: number) => number;
  readonly init_wasm: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0780e095f84217a8: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h2781e30f633460c4: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
