import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model, Msg, Segment } from "./model.js";

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
    view(model);
}


function view(model: Model) {
    console.log("render",model);
    document.querySelector("#layoutroot").addProp({ data: { type: model.type} });
    let main = (document.querySelector("#main") as HTMLDivElement).Clear().addProp({tabindex: 0});
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement).Clear();
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement).Clear();
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement).Clear();
    main.onkeydown = ()=>{}
    const msg = (msg: Msg) => view(update(model,msg));
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
                            main.focus();
                        })
                    ]);
                }
            ))
        );
    }
    if (model.type == "TypingStart") {
        main.Add(elm("h1",{},[textelm(model.content.title)]))
            .Add(elm("h2",{},[textelm("Press Space to start typing")]))
            .Add(elm("h2",{},[textelm("Press Escape to cancel")]));
        main.onkeydown = (e: KeyboardEvent)=>{
                console.log("keydown",e.key);
                if (e.key == " ") {
                    msg({ "TypingStart": "StartTyping" });
                }
                if (e.key == "Escape") {
                    msg({ "TypingStart": "Cancel" });
                }
            };
    }
    if (model.type == "Typing") {
        main.Add(elm("h1",{},[textelm(model.content.title)]))
            .Add(elm("h2",{},model.content.lines[model.status.line].segments.map((seg: Segment,i)=>{
                if (seg.type == "Plain") {
                    return elm("span",{},[textelm(seg.text)]);
                } else if (seg.type == "Annotated") {
                    return elm("ruby",{},[elm("rb",{},[textelm(seg.base)]),elm("rt",{},[textelm(seg.reading)])]);
                }
            })));
        main.onkeydown = (e: KeyboardEvent)=>{
                console.log("keydown",e.key);
            };
    }
}


window.addEventListener("load",init);