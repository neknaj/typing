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

        // File input in sub1 for local file loading
        sub2.Add(elm("h2", {}, [textelm("Load a file from your computer")]));
        sub2.Add(elm("input", { type: "file", id: "file-input" },[]));

        // URL input and button in sub1 for remote file loading via Fetch API
        sub2.Add(elm("h2", {}, [textelm("Or enter a URL to load content")]));
        sub2.Add(elm("input", { type: "text", id: "url-input", placeholder: "Enter URL" },[]));
        sub2.Add(
            elm("button", {}, [textelm("Fetch Content")]).Listen("click", async () => {
                const urlInput = document.getElementById("url-input") as HTMLInputElement;
                const url = urlInput.value;
                if (url) {
                    try {
                        const response = await fetch(url);
                        const text = await response.text();
                        msg({ "Menu": { "AddContent": text } });
                    } catch (error) {
                        console.error("Fetch error:", error);
                    }
                }
            })
        );

        // Add event listener for file input changes
        const fileInput = document.getElementById("file-input") as HTMLInputElement;
        fileInput.addEventListener("change", () => {
            const files = fileInput.files;
            if (files && files.length > 0) {
                const file = files[0];
                const reader = new FileReader();
                reader.onload = (e) => {
                    const text = e.target?.result;
                    msg({ "Menu": { "AddContent": text } });
                };
                reader.readAsText(file);
            }
        });
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
    if (model.type == "Pause") {
        main.Add(elm("h1",{},[textelm("Paused")]))
        .Add(elm("h2",{},[textelm("Press Space to resume typing")]))
        .Add(elm("h2",{},[textelm("Press Escape to Finish")]));
        main.onkeydown = (e: KeyboardEvent)=>{
                console.log("keydown",e.key);
                if (e.key == " ") {
                    msg({ "Pause": "Resume" });
                }
                if (e.key == "Escape") {
                    msg({ "Pause": "Cancel" });
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
                // console.log("keydown",e.key);
                if (e.key == "Escape") {
                    msg({ "Typing": "Pause" });
                }
                else {
                    msg({ "Typing": { "KeyInput": e.key } });
                }
            };
    }
}


window.addEventListener("load",init);