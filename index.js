// src/web/cdom.ts
function elm(type, prop, children) {
  const elm2 = document.createElement(type);
  const propKeys = Object.keys(prop);
  for (const key of propKeys) {
    if (key === "data") {
      for (const k of Object.keys(prop[key])) {
        elm2.dataset[k] = prop[key][k];
      }
    } else {
      elm2.setAttribute(key, prop[key]);
    }
  }
  for (const child of children) {
    elm2.appendChild(child);
  }
  return elm2;
}
function textelm(text) {
  return document.createTextNode(text);
}
Element.prototype.Clear = function() {
  this.innerHTML = "";
  return this;
};
Element.prototype.Add = function(child) {
  this.appendChild(child);
  return this;
};
Element.prototype.Proc = function(func) {
  func(this);
  return this;
};
Element.prototype.Replace = function(children) {
  this.replaceChildren(...children);
  return this;
};
Element.prototype.Listen = function(type, listener, options) {
  this.addEventListener(type, listener, options);
  return this;
};
Element.prototype.addProp = function(prop) {
  const propKeys = Object.keys(prop);
  for (const key of propKeys) {
    if (key === "data") {
      for (const k of Object.keys(prop[key])) {
        this.dataset[k] = prop[key][k];
      }
    } else {
      this.setAttribute(key, prop[key]);
    }
  }
  return this;
};
Element.prototype.addClass = function(name) {
  this.classList.add(name);
  return this;
};

// src/web/layout.js
function setResizer(container, node, callback = () => {
}) {
  const normalizeProp = (arr) => {
    const adj = 100 / arr.reduce((s, x) => {
      return s + x;
    });
    return arr.map((x) => {
      return x * adj;
    });
  };
  const p = container.dataset.proportion.split(":").map((x) => {
    return Number(x);
  });
  node[1] = normalizeProp(p);
  for (const i in node[1]) {
    container.querySelectorAll(":scope > .resizer_content")[i].style.flexBasis = `${node[1][i]}%`;
  }
  container.dataset.proportion = node[1].join(":");
  const type = container.dataset.type == "v";
  Array.from(container.querySelectorAll(":scope > .resizer_splitter")).map((splitter, i) => {
    splitter.addEventListener("pointerdown", (e) => {
      const rect = container.getBoundingClientRect();
      const rects = type ? rect.height : rect.width;
      const rectx = type ? rect.y : rect.x;
      const resizer = container.querySelector(":scope > .resizer_splitter").getBoundingClientRect();
      const resizerW = type ? resizer.height : resizer.width;
      const resize1 = (e2) => {
        const percent = container.dataset.proportion.split(":").map((x) => {
          return Number(x);
        });
        const ex = type ? e2.y : e2.x;
        const width = rects - resizerW * (percent.length - 1);
        const barprop = Math.min(Math.max((ex - rectx) / rects, 1e-4), 0.9999);
        const left = percent.slice(0, i + 1);
        const right = percent.slice(i + 1);
        const leftadj = left.reduce((s, x) => {
          return s + x;
        }, 0) == 0 ? 1 : 1 / left.reduce((s, x) => {
          return s + x;
        }, 0);
        const rightadj = right.reduce((s, x) => {
          return s + x;
        }, 0) == 0 ? 1 : 1 / right.reduce((s, x) => {
          return s + x;
        }, 0);
        const newpercent = normalizeProp(left.map((x) => {
          return x * barprop * leftadj * 100;
        }).concat(right.map((x) => {
          return x * (1 - barprop) * rightadj * 100;
        })).map((x) => {
          return x < 0.01 ? 1 : x;
        }));
        node[1] = newpercent;
        container.dataset.proportion = newpercent.join(":");
        for (const i2 in newpercent) {
          container.querySelectorAll(":scope > .resizer_content")[i2].style.flexBasis = `${newpercent[i2]}%`;
        }
        callback();
        container.dispatchEvent(new CustomEvent("resize"));
      };
      const resize2 = (e2) => {
        const percent = container.dataset.proportion.split(":").map((x) => {
          return Number(x);
        });
        const ex = type ? e2.y : e2.x;
        const width = rects - resizerW * (percent.length - 1);
        const left = percent.slice(0, i + 1 - 1);
        const right = percent.slice(i + 1 + 1);
        const min = left.reduce((s, x) => {
          return s + x;
        }, 0) * rects / 100 + resizerW * i;
        const max = rects - (right.reduce((s, x) => {
          return s + x;
        }, 0) * rects / 100 + resizerW * (percent.length - i - 2));
        const newx = Math.min(Math.max(ex - rectx, min), max);
        if (max - newx < 1 | newx - min < 1) {
          resize1(e2);
          callback();
          return;
        }
        const newpercent = normalizeProp([].concat(left, [(newx - min) * 100 / width], [(max - newx) * 100 / width], right));
        node[1] = newpercent;
        container.dataset.proportion = newpercent.join(":");
        for (const i2 in newpercent) {
          container.querySelectorAll(":scope > .resizer_content")[i2].style.flexBasis = `${newpercent[i2]}%`;
        }
        callback();
        container.dispatchEvent(new CustomEvent("resize"));
      };
      document.addEventListener("pointermove", resize2, false);
      document.addEventListener("pointerup", () => {
        document.removeEventListener("pointermove", resize2, false);
        container.dispatchEvent(new CustomEvent("resizeend"));
      }, false);
      e.target.setPointerCapture(e.pointerId);
    });
  });
  container.dispatchEvent(new CustomEvent("resizestart"));
  return container;
}
function initlayout(elm2, layout) {
  elm2.className = "layout_root";
  elm2.replaceChildren(makeLayoutDOM(layout, "splitlayout"));
  elm2.dispatchEvent(new CustomEvent("doneinitlayout"));
}
function makeLayoutDOM(node, pid) {
  var children = [];
  for (const i in node[2]) {
    if (node[2]?.[i]?.[0] == "h" || node[2]?.[i]?.[0] == "v") {
      children.push(elm("div", { class: "resizer_content" }, [makeLayoutDOM(node[2][i], `${pid}_${i}`)]));
    } else if (node[2]?.[i]?.[0] == "c") {
      children.push(elm("div", { class: "resizer_content" }, [contentarea(node[2][i][1], node[2][i])]));
    } else {
      console.log("");
      node[2][i] = ["c", "empty"];
      children.push(elm("div", { class: "resizer_content" }, [contentarea(node[2][i][1], node[2][i])]));
    }
    if (i < node[2].length - 1) {
      children.push(elm("div", { class: "resizer_splitter" }, []));
    }
  }
  return setResizer(elm("div", { data: { proportion: node[1].join(":"), id: pid, type: node[0] }, class: `resizer_container` }, children), node);
}
var contentarea = (name, node) => {
  return elm("div", { class: "layoutcontentarea" }, [
    elm("div", { class: "layoutcontent", id: name }, [])
  ]);
};

// src/web/snippets/typing-97458ceb71fa37b5/src/web/api.js
async function file_get(file_path) {
  const response = await fetch(file_path);
  if (!response.ok) {
    return { error: response.status };
  }
  const text = await response.text();
  return { value: text };
}
function console_log(json) {
  console.log(...JSON.parse(json));
  return true;
}

// src/web/typing_lib.js
var wasm;
function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_export_2.set(idx, obj);
  return idx;
}
function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    const idx = addToExternrefTable0(e);
    wasm.__wbindgen_exn_store(idx);
  }
}
var cachedTextDecoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
if (typeof TextDecoder !== "undefined") {
  cachedTextDecoder.decode();
}
var cachedUint8ArrayMemory0 = null;
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
function isLikeNone(x) {
  return x === void 0 || x === null;
}
var cachedDataViewMemory0 = null;
function getDataViewMemory0() {
  if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || cachedDataViewMemory0.buffer.detached === void 0 && cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
  }
  return cachedDataViewMemory0;
}
var CLOSURE_DTORS = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((state) => {
  wasm.__wbindgen_export_3.get(state.dtor)(state.a, state.b);
});
function makeMutClosure(arg0, arg1, dtor, f) {
  const state = { a: arg0, b: arg1, cnt: 1, dtor };
  const real = (...args) => {
    state.cnt++;
    const a = state.a;
    state.a = 0;
    try {
      return f(a, state.b, ...args);
    } finally {
      if (--state.cnt === 0) {
        wasm.__wbindgen_export_3.get(state.dtor)(a, state.b);
        CLOSURE_DTORS.unregister(state);
      } else {
        state.a = a;
      }
    }
  };
  real.original = state;
  CLOSURE_DTORS.register(real, state, state);
  return real;
}
function debugString(val) {
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches && builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    return toString.call(val);
  }
  if (className == "Object") {
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  if (val instanceof Error) {
    return `${val.name}: ${val.message}
${val.stack}`;
  }
  return className;
}
var WASM_VECTOR_LEN = 0;
var cachedTextEncoder = typeof TextEncoder !== "undefined" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} };
var encodeString = typeof cachedTextEncoder.encodeInto === "function" ? function(arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function(arg, view) {
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  return {
    read: arg.length,
    written: buf.length
  };
};
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === void 0) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr2 = malloc(buf.length, 1) >>> 0;
    getUint8ArrayMemory0().subarray(ptr2, ptr2 + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr2;
  }
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;
  const mem = getUint8ArrayMemory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 127) break;
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
function init_model(to) {
  const ret = wasm.init_model(to);
  return ret;
}
function add_contents(data) {
  const ret = wasm.add_contents(data);
  return ret;
}
function typing_scroll(data1, data2) {
  wasm.typing_scroll(data1, data2);
}
function event_receive_keyboard(event) {
  wasm.event_receive_keyboard(event);
}
function fetch_render_data() {
  let deferred1_0;
  let deferred1_1;
  try {
    const ret = wasm.fetch_render_data();
    deferred1_0 = ret[0];
    deferred1_1 = ret[1];
    return getStringFromWasm0(ret[0], ret[1]);
  } finally {
    wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
  }
}
function __wbg_adapter_46(arg0, arg1, arg2) {
  wasm.closure35_externref_shim(arg0, arg1, arg2);
}
function __wbg_adapter_92(arg0, arg1, arg2, arg3) {
  wasm.closure57_externref_shim(arg0, arg1, arg2, arg3);
}
async function __wbg_load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        if (module.headers.get("Content-Type") != "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
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
  imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
    const ret = arg0.buffer;
    return ret;
  };
  imports.wbg.__wbg_call_672a4d21634d4a24 = function() {
    return handleError(function(arg0, arg1) {
      const ret = arg0.call(arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_call_7cccdd69e0791ae2 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = arg0.call(arg1, arg2);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_consolelog_03542da212ab3b21 = function(arg0, arg1) {
    const ret = console_log(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbg_done_769e5ede4b31c67b = function(arg0) {
    const ret = arg0.done;
    return ret;
  };
  imports.wbg.__wbg_entries_3265d4158b33e5dc = function(arg0) {
    const ret = Object.entries(arg0);
    return ret;
  };
  imports.wbg.__wbg_fileget_47e32de304acebf5 = function(arg0, arg1) {
    const ret = file_get(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbg_get_67b2ba62fc30de12 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.get(arg0, arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_get_b9b93047fe3cf45b = function(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
  };
  imports.wbg.__wbg_instanceof_ArrayBuffer_e14585432e3737fc = function(arg0) {
    let result;
    try {
      result = arg0 instanceof ArrayBuffer;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_instanceof_Map_f3469ce2244d2430 = function(arg0) {
    let result;
    try {
      result = arg0 instanceof Map;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_instanceof_Uint8Array_17156bcf118086a9 = function(arg0) {
    let result;
    try {
      result = arg0 instanceof Uint8Array;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_isArray_a1eab7e0d067391b = function(arg0) {
    const ret = Array.isArray(arg0);
    return ret;
  };
  imports.wbg.__wbg_isSafeInteger_343e2beeeece1bb0 = function(arg0) {
    const ret = Number.isSafeInteger(arg0);
    return ret;
  };
  imports.wbg.__wbg_iterator_9a24c88df860dc65 = function() {
    const ret = Symbol.iterator;
    return ret;
  };
  imports.wbg.__wbg_length_a446193dc22c12f8 = function(arg0) {
    const ret = arg0.length;
    return ret;
  };
  imports.wbg.__wbg_length_e2d2a49132c1b256 = function(arg0) {
    const ret = arg0.length;
    return ret;
  };
  imports.wbg.__wbg_new_23a2665fac83c611 = function(arg0, arg1) {
    try {
      var state0 = { a: arg0, b: arg1 };
      var cb0 = (arg02, arg12) => {
        const a = state0.a;
        state0.a = 0;
        try {
          return __wbg_adapter_92(a, state0.b, arg02, arg12);
        } finally {
          state0.a = a;
        }
      };
      const ret = new Promise(cb0);
      return ret;
    } finally {
      state0.a = state0.b = 0;
    }
  };
  imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
  };
  imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbg_next_25feadfc0913fea9 = function(arg0) {
    const ret = arg0.next;
    return ret;
  };
  imports.wbg.__wbg_next_6574e1a8a62d1055 = function() {
    return handleError(function(arg0) {
      const ret = arg0.next();
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_now_807e54c39636c349 = function() {
    const ret = Date.now();
    return ret;
  };
  imports.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5 = function(arg0) {
    queueMicrotask(arg0);
  };
  imports.wbg.__wbg_queueMicrotask_d3219def82552485 = function(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
  };
  imports.wbg.__wbg_resolve_4851785c9c5f573d = function(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
  };
  imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
  };
  imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {
    const ret = typeof global === "undefined" ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {
    const ret = typeof globalThis === "undefined" ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {
    const ret = typeof self === "undefined" ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {
    const ret = typeof window === "undefined" ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_then_44b73946d2fb3e7d = function(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
  };
  imports.wbg.__wbg_then_48b406749878a531 = function(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
  };
  imports.wbg.__wbg_value_cd1ffa7b1ab794f1 = function(arg0) {
    const ret = arg0.value;
    return ret;
  };
  imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
    const ret = arg0;
    return ret;
  };
  imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
    const ret = BigInt.asUintN(64, arg0);
    return ret;
  };
  imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
    const v = arg1;
    const ret = typeof v === "bigint" ? v : void 0;
    getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
  };
  imports.wbg.__wbindgen_boolean_get = function(arg0) {
    const v = arg0;
    const ret = typeof v === "boolean" ? v ? 1 : 0 : 2;
    return ret;
  };
  imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
      obj.a = 0;
      return true;
    }
    const ret = false;
    return ret;
  };
  imports.wbg.__wbindgen_closure_wrapper225 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 36, __wbg_adapter_46);
    return ret;
  };
  imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbindgen_in = function(arg0, arg1) {
    const ret = arg0 in arg1;
    return ret;
  };
  imports.wbg.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, void 0);
    table.set(offset + 0, void 0);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
  };
  imports.wbg.__wbindgen_is_bigint = function(arg0) {
    const ret = typeof arg0 === "bigint";
    return ret;
  };
  imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof arg0 === "function";
    return ret;
  };
  imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = arg0;
    const ret = typeof val === "object" && val !== null;
    return ret;
  };
  imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = arg0 === void 0;
    return ret;
  };
  imports.wbg.__wbindgen_json_serialize = function(arg0, arg1) {
    const obj = arg1;
    const ret = JSON.stringify(obj === void 0 ? null : obj);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
    const ret = arg0 === arg1;
    return ret;
  };
  imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
    const ret = arg0 == arg1;
    return ret;
  };
  imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return ret;
  };
  imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof obj === "number" ? obj : void 0;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
  };
  imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof obj === "string" ? obj : void 0;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
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
  wasm.__wbindgen_start();
  return wasm;
}
async function __wbg_init(module_or_path) {
  if (wasm !== void 0) return wasm;
  if (typeof module_or_path !== "undefined") {
    if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
      ({ module_or_path } = module_or_path);
    } else {
      console.warn("using deprecated parameters for the initialization function; pass a single object instead");
    }
  }
  if (typeof module_or_path === "undefined") {
    module_or_path = new URL("typing_lib_bg.wasm", import.meta.url);
  }
  const imports = __wbg_get_imports();
  if (typeof module_or_path === "string" || typeof Request === "function" && module_or_path instanceof Request || typeof URL === "function" && module_or_path instanceof URL) {
    module_or_path = fetch(module_or_path);
  }
  __wbg_init_memory(imports);
  const { instance, module } = await __wbg_load(await module_or_path, imports);
  return __wbg_finalize_init(instance, module);
}
var typing_lib_default = __wbg_init;

// src/web/index.ts
async function init() {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const layout = urlParams.get("layout") == "h" ? "h" : "v";
  await typing_lib_default();
  document.querySelector("#layoutroot").addProp({ tabindex: 0 }).Listen("keydown", (e) => {
    event_receive_keyboard(e.key);
  });
  await init_model(layout == "h" ? "Horizontal" : "Vertical");
  render();
  const dropzone = document.querySelector("#layoutroot");
  ["dragenter", "dragover", "dragleave", "drop"].forEach((eventName) => {
    dropzone.addEventListener(eventName, (e) => {
      e.preventDefault();
      e.stopPropagation();
    }, false);
  });
  dropzone.addEventListener("drop", (e) => {
    const files = e.dataTransfer.files;
    if (files.length > 0) {
      const file = files[0];
      const reader = new FileReader();
      reader.onload = function(event) {
        console.log(event.target.result);
        add_contents(event.target.result);
      };
      reader.readAsText(file);
    }
  });
}
window.addEventListener("load", init);
var lastFpsUpdate = performance.now();
var frameCount = 0;
var fps = 0;
function render() {
  let data = JSON.parse(fetch_render_data());
  let now = performance.now();
  frameCount++;
  if (now - lastFpsUpdate >= 1e3) {
    fps = frameCount;
    frameCount = 0;
    lastFpsUpdate = now;
    document.querySelector("#overlay").Clear().Add(elm("p", {}, [textelm("FPS: "), textelm(fps.toString())]));
  }
  if (data[0] == "Menu") {
    let selecting = data[1];
    let menu = data[2];
    let layout = data[3];
    let [main, sub1, sub2, sub3] = layout_switch(layout);
    main.Clear();
    sub1.Clear();
    sub2.Clear();
    sub3.Clear();
    main.Add(elm("h1", {}, [textelm("Neknaj Typing Game")])).Add(
      elm("ul", {}, menu.map(
        (content, i) => {
          let e = elm("li", {}, [
            textelm(content)
          ]);
          if (i == selecting) {
            e.classList.add("selecting");
          }
          return e;
        }
      ))
    );
  }
  if (data[0] == "TypingStart") {
    let title = data[1];
    let layout = data[2];
    let [main, sub1, sub2, sub3] = layout_switch(layout);
    main.Clear();
    sub1.Clear();
    sub2.Clear();
    sub3.Clear();
    main.Add(elm("h1", {}, [textelm(title)])).Add(elm("p", {}, [textelm("Press Space to start typing")])).Add(elm("p", {}, [textelm("Press Escape to cancel")]));
    let text_orientation = data[2];
    if (text_orientation == "Horizontal") {
      let w = main.getBoundingClientRect().width;
      typing_scroll(-w, -w);
    } else {
      let w = main.getBoundingClientRect().height;
      typing_scroll(-w, -w);
    }
  }
  if (data[0] == "Typing") {
    let title = data[1];
    let segments = data[2];
    let correct = data[3];
    let status = data[4];
    let text_orientation = data[5];
    let scroll = data[6];
    let metrics = data[7];
    let segment = segments[status.segment];
    console.log(metrics);
    let [main, sub1, sub2, sub3] = layout_switch(text_orientation);
    main.Clear();
    sub1.Clear();
    sub2.Clear();
    sub3.Clear();
    main.Add(elm("h1", {}, [textelm(title)])).Add(elm("br", {}, [])).Add(elm("div", { class: "typing_scroll" }, [
      elm("p", { class: "typing" }, segments.map((seg, i) => {
        if (seg.type == "Plain") {
          return elm("span", {}, [textelm(seg.text)]);
        } else if (seg.type == "Annotated") {
          return elm("ruby", {}, [elm("rb", {}, [textelm(seg.base)]), elm("rt", {}, [textelm(seg.reading)])]);
        }
      })),
      elm(
        "p",
        { class: "typing" },
        [
          ...segments.slice(0, status.segment).map((seg, si) => {
            if (seg.type == "Plain") {
              return elm(
                "span",
                { class: "plain" },
                seg.text.split("").map((c, ci) => elm("span", { class: correct[si].chars[ci] }, [textelm(c)]))
              );
            } else if (seg.type == "Annotated") {
              return elm("ruby", { class: "annotated" }, [
                elm(
                  "rb",
                  {
                    class: seg.reading.split("").map((c, ci) => correct[si].chars[ci]).includes("Incorrect") ? "Incorrect" : "Correct"
                  },
                  [textelm(seg.base)]
                ),
                elm(
                  "rt",
                  {},
                  seg.reading.split("").map((c, ci) => elm("span", { class: correct[si].chars[ci] }, [textelm(c)]))
                )
              ]);
            }
          }),
          elm(
            "span",
            { class: "pendingSegment" },
            (segment.type == "Annotated" ? segment.reading : segment.text).slice(0, status.char_).split("").map((c, ci) => elm("span", { class: correct[status.segment].chars[ci] }, [textelm(c)]))
          ),
          elm("span", { class: "unconfirmed" }, [textelm(status.unconfirmed.join(""))]),
          elm("span", { class: "cursor" }, []),
          elm("span", { class: "wrong" }, [textelm(status.last_wrong_keydown != null ? status.last_wrong_keydown : "")])
        ]
      )
    ]));
    const elements = document.querySelectorAll(".plain, .annotated");
    const lastElement = elements[elements.length - 1];
    let anchor1 = (elements.length > 0 ? lastElement : document.querySelector(".pendingSegment")).getBoundingClientRect();
    let anchor2 = document.querySelector(".cursor").getBoundingClientRect();
    if (text_orientation == "Horizontal") {
      let w = main.getBoundingClientRect().width;
      let target = w * 0.3;
      typing_scroll((anchor1.x + anchor2.x * 3) / 4 - target, -w);
      document.querySelector(".typing_scroll").style.setProperty("--scroll", `${-scroll}px`);
    } else {
      let w = main.getBoundingClientRect().height;
      let target = w * 0.3;
      typing_scroll((anchor1.y + anchor2.y * 3) / 4 - target, -w);
      document.querySelector(".typing_scroll").style.setProperty("--scroll", `${-scroll}px`);
    }
    sub3.Add(
      elm("div", { class: "metrics" }, [
        elm("table", {}, [
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Accuracy")]),
            elm("td", {}, [textelm(`: ${(metrics.accuracy * 100).toFixed(2)}`)]),
            elm("td", {}, [textelm(`%`)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Speed")]),
            elm("td", {}, [textelm(`: ${metrics.speed.toFixed(2)}`)]),
            elm("td", {}, [textelm(`chars/sec`)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Miss Count")]),
            elm("td", {}, [textelm(`: ${metrics.miss_count}`)]),
            elm("td", {}, [textelm(``)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Type Count")]),
            elm("td", {}, [textelm(`: ${metrics.type_count}`)]),
            elm("td", {}, [textelm(``)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Time")]),
            elm("td", {}, [textelm(`: ${(metrics.total_time / 1e3).toFixed(1)}`)]),
            elm("td", {}, [textelm(`sec`)])
          ])
        ])
      ])
    );
  }
  if (data[0] == "Result") {
    let title = data[1];
    let metrics = data[2];
    let layout = data[3];
    let [main, sub1, sub2, sub3] = layout_switch(layout);
    main.Clear();
    sub1.Clear();
    sub2.Clear();
    sub3.Clear();
    main.Add(elm("h1", {}, [textelm("Result")])).Add(elm("h2", {}, [textelm(title)])).Add(elm("p", {}, [textelm("Press Space to Restart typing")])).Add(elm("p", {}, [textelm("Press Escape to Back to Menu")]));
    sub3.Add(
      elm("div", { class: "metrics" }, [
        elm("table", {}, [
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Accuracy")]),
            elm("td", {}, [textelm(`: ${(metrics.accuracy * 100).toFixed(2)}`)]),
            elm("td", {}, [textelm(`%`)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Speed")]),
            elm("td", {}, [textelm(`: ${metrics.speed.toFixed(2)}`)]),
            elm("td", {}, [textelm(`chars/sec`)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Miss Count")]),
            elm("td", {}, [textelm(`: ${metrics.miss_count}`)]),
            elm("td", {}, [textelm(``)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Type Count")]),
            elm("td", {}, [textelm(`: ${metrics.type_count}`)]),
            elm("td", {}, [textelm(``)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Time")]),
            elm("td", {}, [textelm(`: ${(metrics.total_time / 1e3).toFixed(1)}`)]),
            elm("td", {}, [textelm(`sec`)])
          ])
        ])
      ])
    );
  }
  if (data[0] == "Pause") {
    let title = data[1];
    let metrics = data[2];
    let layout = data[3];
    let [main, sub1, sub2, sub3] = layout_switch(layout);
    main.Clear();
    sub1.Clear();
    sub2.Clear();
    sub3.Clear();
    main.Add(elm("h1", {}, [textelm("Pause")])).Add(elm("h2", {}, [textelm(title)])).Add(elm("p", {}, [textelm("Press Space to Resume typing")])).Add(elm("p", {}, [textelm("Press Escape to Finish")]));
    sub3.Add(
      elm("div", { class: "metrics" }, [
        elm("table", {}, [
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Accuracy")]),
            elm("td", {}, [textelm(`: ${(metrics.accuracy * 100).toFixed(2)}`)]),
            elm("td", {}, [textelm(`%`)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Speed")]),
            elm("td", {}, [textelm(`: ${metrics.speed.toFixed(2)}`)]),
            elm("td", {}, [textelm(`chars/sec`)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Miss Count")]),
            elm("td", {}, [textelm(`: ${metrics.miss_count}`)]),
            elm("td", {}, [textelm(``)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Type Count")]),
            elm("td", {}, [textelm(`: ${metrics.type_count}`)]),
            elm("td", {}, [textelm(``)])
          ]),
          elm("tr", {}, [
            elm("td", { class: "metric-label" }, [textelm("Time")]),
            elm("td", {}, [textelm(`: ${(metrics.total_time / 1e3).toFixed(1)}`)]),
            elm("td", {}, [textelm(`sec`)])
          ])
        ])
      ])
    );
  }
  requestAnimationFrame(render);
}
function layout_switch(layout) {
  let h = layout == "Horizontal";
  if (document.querySelector("html").dataset.layout == layout) {
    let main2 = document.querySelector("#main");
    let sub12 = document.querySelector("#sub1");
    let sub22 = document.querySelector("#sub2");
    let sub32 = document.querySelector("#sub3");
    return [main2, sub12, sub22, sub32];
  }
  document.querySelector("html").dataset.layout = layout;
  initlayout(
    document.querySelector("#layoutroot"),
    ["h", [5, 3], [
      [h ? "v" : "h", h ? [3, 2] : [2, 3], [
        ["c", h ? "main" : "sub1"],
        ["c", h ? "sub1" : "main"]
      ]],
      ["v", [2, 5], [
        ["c", "sub2"],
        ["c", "sub3"]
      ]]
    ]]
  );
  let main = document.querySelector("#main");
  let sub1 = document.querySelector("#sub1");
  let sub2 = document.querySelector("#sub2");
  let sub3 = document.querySelector("#sub3");
  return [main, sub1, sub2, sub3];
}
