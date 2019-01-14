/* tslint:disable *//* eslint-disable */
import * as wasm from './csv2sql_bg';

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_alert_2d30aae8e03ae5fd(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}

export function __wbg_log_03472dbc351a4f29(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    console.log(varg0);
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
}
/**
* @param {string} arg0
* @returns {void}
*/
export function greet(arg0) {
    const ptr0 = passStringToWasm(arg0);
    const len0 = WASM_VECTOR_LEN;
    try {
        return wasm.greet(ptr0, len0);

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* @param {string} arg0
* @returns {any}
*/
export function get_columns(arg0) {
    const ptr0 = passStringToWasm(arg0);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.get_columns(ptr0, len0));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

/**
* @param {string} arg0
* @param {string} arg1
* @returns {any}
*/
export function check_correction(arg0, arg1) {
    const ptr0 = passStringToWasm(arg0);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm(arg1);
    const len1 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.check_correction(ptr0, len0, ptr1, len1));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        wasm.__wbindgen_free(ptr1, len1 * 1);

    }

}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @param {string} arg0
* @param {any} arg1
* @returns {any}
*/
export function process_file(arg0, arg1) {
    const ptr0 = passStringToWasm(arg0);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.process_file(ptr0, len0, addHeapObject(arg1)));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

/**
* @param {string} arg0
* @param {any} arg1
* @param {any} arg2
* @returns {any}
*/
export function generate_file(arg0, arg1, arg2) {
    const ptr0 = passStringToWasm(arg0);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.generate_file(ptr0, len0, addHeapObject(arg1), addHeapObject(arg2)));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

export function __wbg_self_1ec1c3e6d75f31d7(arg0) {
    return addHeapObject(getObject(arg0).self);
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

export function __wbg_stack_27fca1c99e84b66f(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).stack);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __wbg_static_accessor_document_b0c76025fc904ba5() {
    return addHeapObject(document);
}

export function __wbg_getElementById_b568d9288b16b48f(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(getObject(arg0).getElementById(varg1));
}

export function __wbg_innerhtml_12e39d90d691cea1(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).innerHTML);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __wbg_setinnerhtml_bc8921a1ce6b1b82(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).innerHTML = varg1;
}

export function __wbg_stack_a8569ef64277dd4e(arg0) {
    return addHeapObject(getObject(arg0).stack);
}

export function __wbg_log_74d904c418cf24db(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    console.log(varg0);
}

export function __wbg_String_5380427aff8fe7c7(ret, arg0) {

    const retptr = passStringToWasm(String(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

function passArrayJsValueToWasm(array) {
    const ptr = wasm.__wbindgen_malloc(array.length * 4);
    const mem = getUint32Memory();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* Handler for `console.log` invocations.
*
* If a test is currently running it takes the `args` array and stringifies
* it and appends it to the current output of the test. Otherwise it passes
* the arguments to the original `console.log` function, psased as
* `original`.
* @param {any} arg0
* @param {any} arg1
* @returns {void}
*/
export function __wbgtest_console_log(arg0, arg1) {
    try {
        return wasm.__wbgtest_console_log(addBorrowedObject(arg0), addBorrowedObject(arg1));

    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;

    }

}

/**
* Handler for `console.error` invocations.
*
* Works the same as `console_log` above.
* @param {any} arg0
* @param {any} arg1
* @returns {void}
*/
export function __wbgtest_console_error(arg0, arg1) {
    try {
        return wasm.__wbgtest_console_error(addBorrowedObject(arg0), addBorrowedObject(arg1));

    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;

    }

}

export function __wbg_forEach_7796ea23499aa552(arg0, arg1, arg2) {
    let cbarg1 = function(arg0, arg1, arg2) {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b, addHeapObject(arg0), arg1, addHeapObject(arg2));

        } finally {
            this.a = a;

        }

    };
    cbarg1.f = wasm.__wbg_function_table.get(142);
    cbarg1.a = arg1;
    cbarg1.b = arg2;
    try {
        getObject(arg0).forEach(cbarg1.bind(cbarg1));
    } finally {
        cbarg1.a = cbarg1.b = 0;

    }
}

export function __wbg_message_0ff90cbb5270c176(arg0) {
    return addHeapObject(getObject(arg0).message);
}

export function __wbg_name_dddf7a57ae3f9867(arg0) {
    return addHeapObject(getObject(arg0).name);
}

export function __wbg_newnoargs_a6ad1b52f5989ea9(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

export function __wbg_apply_823defaa2a295bb4(arg0, arg1, arg2, exnptr) {
    try {
        return addHeapObject(getObject(arg0).apply(getObject(arg1), getObject(arg2)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbg_call_720151a19a4c6808(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbg_call_7aced47e67a8c62d(arg0, arg1, arg2, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1), getObject(arg2)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbg_new_bdd94b8735e4f66d(arg0, arg1) {
    let cbarg0 = function(arg0, arg1) {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b, addHeapObject(arg0), addHeapObject(arg1));

        } finally {
            this.a = a;

        }

    };
    cbarg0.f = wasm.__wbg_function_table.get(135);
    cbarg0.a = arg0;
    cbarg0.b = arg1;
    try {
        return addHeapObject(new Promise(cbarg0.bind(cbarg0)));
    } finally {
        cbarg0.a = cbarg0.b = 0;

    }
}

export function __wbg_resolve_b2d9398056dbfe64(arg0) {
    return addHeapObject(Promise.resolve(getObject(arg0)));
}

export function __wbg_then_c75e723ffb976395(arg0, arg1) {
    return addHeapObject(getObject(arg0).then(getObject(arg1)));
}

export function __wbg_error_cc95a3d302735ca3(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);

    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);

    console.error(varg0);
}

function freeContext(ptr) {

    wasm.__wbg_context_free(ptr);
}
/**
* Runtime test harness support instantiated in JS.
*
* The node.js entry script instantiates a `Context` here which is used to
* drive test execution.
*/
export class Context {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeContext(ptr);
    }

    /**
    * Creates a new context ready to run tests.
    *
    * A `Context` is the main structure through which test execution is
    * coordinated, and this will collect output and results for all executed
    * tests.
    * @returns {}
    */
    constructor() {
        this.ptr = wasm.context_new();
    }
    /**
    * Inform this context about runtime arguments passed to the test
    * harness.
    *
    * Eventually this will be used to support flags, but for now it\'s just
    * used to support test filters.
    * @param {any[]} arg0
    * @returns {void}
    */
    args(arg0) {
        const ptr0 = passArrayJsValueToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return wasm.context_args(this.ptr, ptr0, len0);
    }
    /**
    * Executes a list of tests, returning a promise representing their
    * eventual completion.
    *
    * This is the main entry point for executing tests. All the tests passed
    * in are the JS `Function` object that was plucked off the
    * `WebAssembly.Instance` exports list.
    *
    * The promise returned resolves to either `true` if all tests passed or
    * `false` if at least one test failed.
    * @param {any[]} arg0
    * @returns {any}
    */
    run(arg0) {
        const ptr0 = passArrayJsValueToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return takeObject(wasm.context_run(this.ptr, ptr0, len0));
    }
}

export function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

export function __wbindgen_number_new(i) { return addHeapObject(i); }

export function __wbindgen_string_get(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const ptr = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
    return ptr;
}

export function __wbindgen_cb_drop(i) {
    const obj = getObject(i).original;
    dropObject(i);
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}

export function __wbindgen_json_parse(ptr, len) {
    return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));
}

export function __wbindgen_json_serialize(idx, ptrptr) {
    const ptr = passStringToWasm(JSON.stringify(getObject(idx)));
    getUint32Memory()[ptrptr / 4] = ptr;
    return WASM_VECTOR_LEN;
}

export function __wbindgen_jsval_eq(a, b) {
    return getObject(a) === getObject(b) ? 1 : 0;
}

export function __wbindgen_closure_wrapper2934(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(113);
    const d = wasm.__wbg_function_table.get(114);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            this.a = a;
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

