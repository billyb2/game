let wasm_bindgen;(function(){function a(a){m===l.length&&l.push(l.length+1);const b=m;return m=l[b],l[b]=a,b}function b(a){return l[a]}function c(a){36>a||(l[a]=m,m=a)}function d(a){const d=b(a);return c(a),d}function e(){return(null===o||o.buffer!==k.memory.buffer)&&(o=new Uint8Array(k.memory.buffer)),o}function f(a,b){return n.decode(e().subarray(a,a+b))}function g(a,b){const c=b(1*a.length);return e().set(a,c/1),p=a.length,c}async function h(a,b){if("function"==typeof Response&&a instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if("application/wasm"!=a.headers.get("Content-Type"))console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",b);else throw b}const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);return c instanceof WebAssembly.Instance?{instance:c,module:a}:c}}async function i(c){if("undefined"==typeof c){let a;a="undefined"==typeof document?location.href:document.currentScript.src,c=a.replace(/\.js$/,"_bg.wasm")}const e={};e.wbg={},e.wbg.__wbindgen_memory=function(){var b=k.memory;return a(b)},e.wbg.__wbg_buffer_79a3294266d4e783=function(c){var d=b(c).buffer;return a(d)},e.wbg.__wbg_newwithbyteoffsetandlength_22a36e6023ad3cd0=function(c,d,e){var f=new Uint8Array(b(c),d>>>0,e>>>0);return a(f)},e.wbg.__wbindgen_object_drop_ref=function(a){d(a)},e.wbg.__wbg_new_945397fb09fec0b8=function(c){var d=new Uint8Array(b(c));return a(d)},e.wbg.__wbindgen_throw=function(a,b){throw new Error(f(a,b))},("string"==typeof c||"function"==typeof Request&&c instanceof Request||"function"==typeof URL&&c instanceof URL)&&(c=fetch(c));const{instance:g,module:j}=await h(await c,e);return k=g.exports,i.__wbindgen_wasm_module=j,k.__wbindgen_start(),k}const j={};let k;const l=Array(32).fill(void 0);l.push(void 0,null,!0,!1);let m=l.length,n=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});n.decode();let o=null;j.main=function(){k.main()};let p=0;j.decompress=function(a){var b=g(a,k.__wbindgen_malloc),c=p,e=k.decompress(b,c);return d(e)},wasm_bindgen=Object.assign(i,j)})();