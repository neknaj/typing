import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model, Msg } from "./model.js";

import initWasm, { new_model, update } from './typing_lib.js';

async function init() {
    await initWasm(); // Wasmモジュールの初期化
    initlayout(
        document.querySelector("#layoutroot"),
        ["h",[5,2],[
            ["v",[2,1],[
                ["c","main"],
                ["c","sub1"],
            ]],
            ["v",[3,3],[
                ["c","sub2"],
                ["c","sub3"],
            ]]
        ]],
    );
    let model = await new_model() as Model;
    render(model);
}


function render(model: Model) {
    console.log("render",model);
    document.querySelector("#layoutroot").addProp({ data: { type: model.type} });
    let main = (document.querySelector("#main") as HTMLDivElement).Clear();
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement).Clear();
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement).Clear();
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement).Clear();
    const msg = (msg: Msg) => render(update(model,msg));
    if (model.type == "Menu") {
        main.Add(elm("h1",{},[textelm("Neknaj Typing Game")])).Add(
            elm("ul", {}, model.available_contents.map(
                (content) => {
                    return elm("li", {}, [
                        elm("button", {}, [
                            textelm(content.title),
                        ]).Listen("click", async () => {
                            let Msg_SelectContent: Msg = {
                                "Menu": {
                                    "SelectContent": content,
                                },
                            };
                            msg(Msg_SelectContent);
                        })
                    ]);
                }
            ))
        );
    }
    if (model.type == "TypingStart") {
        main.Add(elm("h1",{},[textelm(model.content.title)]))
            .Add(elm("h2",{},[textelm("Press Space to start typing")]));
    }
}


window.addEventListener("load",init);