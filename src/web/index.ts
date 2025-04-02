import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model } from "./model.js";

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
    let model = await new_model() as Model;
    console.log(model);
    render(model);
}


function render(model: Model) {
    console.log(model.type);
}


window.addEventListener("load",init);