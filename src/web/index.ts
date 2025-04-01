import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";

import initWasm, { greet } from './typing_lib.js';

async function init() {
    await initWasm(); // Wasmモジュールの初期化
    console.log(greet('World')); // Rustで書いた関数を呼び出し
    initlayout(
        document.querySelector("#layoutroot"),
        ["h",[5,2],[
            ["v",[2,1],[
                ["c","moduleInfo"],
                ["c","tsTranspiled"],
            ]],
            ["v",[3,3],[
                ["c","errMsgArea"],
                ["c","testResult"],
            ]]
        ]],
    );
    render();
}


function render() {
}


window.addEventListener("load",init);