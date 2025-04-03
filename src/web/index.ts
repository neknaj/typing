import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model, Msg, Segment } from "./model.js";

import initWasm, { new_model, update } from './typing_lib.js';

async function init() {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    const layout = urlParams.get('layout')?urlParams.get('layout'):"h";
    await initWasm(); // Wasmモジュールの初期化
    document.querySelector("html").dataset.layout = layout;
    initlayout(
        document.querySelector("#layoutroot"),
        ["h",[5,3],[
            [layout=="h"?"v":"h",[1,1],[
                ["c",layout=="h"?"main":"sub1"],
                ["c",layout=="h"?"sub1":"main"],
            ]],
            ["v",[2,5],[
                ["c","sub2"],
                ["c","sub3"],
            ]]
        ]],
    );
    let model = await new_model() as Model;
    const msg = (msg: Msg) => view(update(model,msg),layout=="h"?"h":"v");
    { // irohaを追加
        const response = await fetch("./examples/iroha.ntq");
        const text = await response.text();
        msg({ "Menu": { "AddContent": text } });
    }
}


function view(model: Model,layout: "v"|"h") {
    console.log("render",model);
    document.querySelector("#layoutroot").addProp({ data: { type: model.type} });
    let main = (document.querySelector("#main") as HTMLDivElement).Clear().addProp({tabindex: 0});
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement).Clear();
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement).Clear();
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement).Clear();
    main.onkeydown = ()=>{}
    const msg = (msg: Msg) => view(update(model,msg),layout);
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
            .Add(elm("p",{},[textelm("Press Space to start typing")]))
            .Add(elm("p",{},[textelm("Press Escape to cancel")]));
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
        .Add(elm("p",{},[textelm("Press Space to resume typing")]))
        .Add(elm("p",{},[textelm("Press Escape to Finish")]));
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
    if (model.type == "Result") {
        main.Add(elm("h1",{},[textelm("Result")]))
        .Add(elm("p",{},[textelm("Press Escape to back to menu")]))
        .Add(elm("p",{},[textelm("Press space to retry")]));
        main.onkeydown = (e: KeyboardEvent)=>{
                console.log("keydown",e.key);
                if (e.key == "Escape") {
                    msg({ "Result": "BackToMenu" });
                }
                if (e.key == " ") {
                    msg({ "Result": "Retry" });
                }
            };
        sub1.Add(elm("div",{},model.typing_model.content.lines.slice(0,model.typing_model.status.line).map((line,li)=>{
            return elm("p",{},line.segments.map((seg: Segment,si)=>{
                if (seg.type == "Plain") {
                    return elm("span",{class:"plain"},
                        seg.text.split("").map((c,ci)=>elm("span",{class:model.typing_model.typing_correctness.lines[li].segments[si].chars[ci].type},[textelm(c)]))
                    );
                } else if (seg.type == "Annotated") {
                    return elm("ruby",{class:"annotated"},[
                        elm("rb",{
                                class:seg.reading.split("").map((c,ci)=>model.typing_model.typing_correctness.lines[li].segments[si].chars[ci].type).includes("Incorrect")?"Incorrect":"Correct"
                            },
                            [textelm(seg.base)]
                        ),
                        elm("rt",{},
                            seg.reading.split("")
                                .map((c,ci)=>elm("span",{class:model.typing_model.typing_correctness.lines[li].segments[si].chars[ci].type},[textelm(c)]))
                        ),
                    ]);
                }
            }));
        })));
    }
    if (model.type == "Typing") {
        let segment = model.content.lines[model.status.line].segments[model.status.segment];
        main.Add(elm("h1",{},[textelm(model.content.title)])).Add(elm("br",{},[]))
            .Add(elm("p",{class:"typing"},model.content.lines[model.status.line].segments.map((seg: Segment,i)=>{
                if (seg.type == "Plain") {
                    return elm("span",{},[textelm(seg.text)]);
                } else if (seg.type == "Annotated") {
                    return elm("ruby",{},[elm("rb",{},[textelm(seg.base)]),elm("rt",{},[textelm(seg.reading)])]);
                }
            })))
            .Add(elm("p",{class:"typing"},
                [
                    ...model.content.lines[model.status.line].segments.slice(0,model.status.segment).map((seg: Segment,si)=>{
                        if (seg.type == "Plain") {
                            return elm("span",{class:"plain"},
                                seg.text.split("").map((c,ci)=>elm("span",{class:model.typing_correctness.lines[model.status.line].segments[si].chars[ci].type},[textelm(c)]))
                            );
                        } else if (seg.type == "Annotated") {
                            return elm("ruby",{class:"annotated"},[
                                elm("rb",{
                                        class:seg.reading.split("").map((c,ci)=>model.typing_correctness.lines[model.status.line].segments[si].chars[ci].type).includes("Incorrect")?"Incorrect":"Correct"
                                    },
                                    [textelm(seg.base)]
                                ),
                                elm("rt",{},
                                    seg.reading.split("")
                                        .map((c,ci)=>elm("span",{class:model.typing_correctness.lines[model.status.line].segments[si].chars[ci].type},[textelm(c)]))
                                ),
                            ]);
                        }
                    }),
                    elm("span",{class:"pendingSegment"},
                        (segment.type=="Annotated"?segment.reading:segment.text).slice(0,model.status.char_)
                            .split("")
                            .map((c,ci)=>elm("span",{class:model.typing_correctness.lines[model.status.line].segments[model.status.segment].chars[ci].type},[textelm(c)]))
                    ),
                    elm("span",{},[textelm(model.status.unconfirmed.join(""))]),
                    elm("span",{class: "cursor"},[]),
                    elm("span",{class: "wrong"},[textelm(model.status.last_wrong_keydown!=null?model.status.last_wrong_keydown:"")]),
                ]
            ));
            sub1.Add(elm("div",{},model.content.lines.slice(0,model.status.line).map((line,li)=>{
                return elm("p",{},line.segments.map((seg: Segment,si)=>{
                    if (seg.type == "Plain") {
                        return elm("span",{class:"plain"},
                            seg.text.split("").map((c,ci)=>elm("span",{class:model.typing_correctness.lines[li].segments[si].chars[ci].type},[textelm(c)]))
                        );
                    } else if (seg.type == "Annotated") {
                        return elm("ruby",{class:"annotated"},[
                            elm("rb",{
                                    class:seg.reading.split("").map((c,ci)=>model.typing_correctness.lines[li].segments[si].chars[ci].type).includes("Incorrect")?"Incorrect":"Correct"
                                },
                                [textelm(seg.base)]
                            ),
                            elm("rt",{},
                                seg.reading.split("")
                                    .map((c,ci)=>elm("span",{class:model.typing_correctness.lines[li].segments[si].chars[ci].type},[textelm(c)]))
                            ),
                        ]);
                    }
                }));
            })));
            sub1.Add(
                elm("div", { class: "activeline" }, [model.content.lines[model.status.line]].map((line) => {
                    return elm("p", {}, [
                        // Add the triangle marker at the start
                        elm("span", { class: "triangle" }, [textelm(layout=="h"?"▷":"▽")]),
                        // Then add the rest of the line segments inline
                        ...line.segments.map((seg: Segment, i) => {
                            if (seg.type == "Plain") {
                                return elm("span", { class: "Pending" }, [textelm(seg.text)]);
                            } else if (seg.type == "Annotated") {
                                return elm("ruby", { class: "Pending" }, [
                                    elm("rb", {}, [textelm(seg.base)]),
                                    elm("rt", {}, [textelm(seg.reading)])
                                ]);
                            }
                        })
                    ]);
                }))
            );
            sub1.Add(elm("div",{},model.content.lines.slice(model.status.line+1).map((line)=>{
                return elm("p",{},line.segments.map((seg: Segment,i)=>{
                    if (seg.type == "Plain") {
                        return elm("span",{class:"Pending"},[textelm(seg.text)]);
                    } else if (seg.type == "Annotated") {
                        return elm("ruby",{class:"Pending"},[elm("rb",{},[textelm(seg.base)]),elm("rt",{},[textelm(seg.reading)])]);
                    }
                }));
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