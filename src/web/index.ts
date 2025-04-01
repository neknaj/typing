import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";

import initWasm, { new_model, update } from './typing_lib.js';

async function init() {
    await initWasm(); // Wasmモジュールの初期化
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
    console.log(await new_model());
    render();
}


function render() {
}


window.addEventListener("load",init);