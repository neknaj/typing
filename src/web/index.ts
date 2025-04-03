import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model, Msg, Segment, TextOrientation, TypingCorrectnessSegment, TypingStatus } from "./model.js";

import initWasm, { init_model, event_receive_keyboard, fetch_render_data, add_contents } from './typing_lib.js';


async function init() {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    const layout = urlParams.get('layout')=="h"?"h":"v";
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
    await init_model(layout=="h"?"Horizontal":"Vertical" as TextOrientation);
    let main = (document.querySelector("#main") as HTMLDivElement).Clear().addProp({tabindex: 0});
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement).Clear();
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement).Clear();
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement).Clear();
    main.Listen("keydown",(e: KeyboardEvent)=>{
        event_receive_keyboard(e.key);
    })
    render();

    const dropzone = document.querySelector("#layoutroot");
    ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
        dropzone.addEventListener(eventName, (e) => {
            e.preventDefault();
            e.stopPropagation();
        }, false);
    });
    // Handle drop event
    dropzone.addEventListener('drop', (e) => {
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            const file = files[0];
            // Only proceed if file is a text file
                const reader = new FileReader();
                reader.onload = function(event) {
                    // Log the file content to console
                    console.log(event.target.result);
                    add_contents(event.target.result);
                };
                reader.readAsText(file);
        }
    });
}

window.addEventListener("load",init);

// Variables for tracking time and frame count
let lastFpsUpdate = performance.now();  // Last time FPS was updated
let frameCount = 0;                     // Counter for rendered frames
let fps = 0;                            // Current FPS value

function render() {
    let data = JSON.parse(fetch_render_data());

    let main = (document.querySelector("#main") as HTMLDivElement).Clear();
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement).Clear();
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement);
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement).Clear();

    let now = performance.now();
    frameCount++;
    // Update the FPS value every 1000 milliseconds (1 second)
    if (now - lastFpsUpdate >= 1000) {
        fps = frameCount;
        frameCount = 0;
        lastFpsUpdate = now;
        sub2.Clear().Add(elm("p",{},[textelm("FPS: "),textelm(fps.toString())]))
    }
    if (data[0] == "Menu") {
        let menu = data[2];
        main.Add(elm("h1",{},[textelm("Neknaj Typing Game")])).Add(
            elm("ul", {}, menu.map(
                (content,i) => {
                    let e =elm("li", {}, [
                        textelm(content)
                    ]);
                    if (i==data[1]) {
                        e.classList.add("selecting")
                    }
                    return e;
                }
            )))
    }
    if (data[0] == "TypingStart") {
        main.Add(elm("h1",{},[textelm(data[1])]))
            .Add(elm("p",{},[textelm("Press Space to start typing")]))
            .Add(elm("p",{},[textelm("Press Escape to cancel")]));
    }
    if (data[0] == "Typing") {
        let title = data[1] as string;
        let segments = data[2] as Segment[];
        let correct = data[3] as TypingCorrectnessSegment[];
        let status = data[4] as TypingStatus;
        let segment = segments[status.segment];
        // console.log(data)
        main.Add(elm("h1",{},[textelm(title)])).Add(elm("br",{},[]))
        .Add(elm("div",{class:"typing_scroll"},[
            elm("p",{class:"typing"},segments.map((seg: Segment,i)=>{
                if (seg.type == "Plain") {
                    return elm("span",{},[textelm(seg.text)]);
                } else if (seg.type == "Annotated") {
                    return elm("ruby",{},[elm("rb",{},[textelm(seg.base)]),elm("rt",{},[textelm(seg.reading)])]);
                }
            })),
            elm("p",{class:"typing"},
                [
                    ...segments.slice(0,status.segment).map((seg: Segment,si)=>{
                        if (seg.type == "Plain") {
                            return elm("span",{class:"plain"},
                                seg.text.split("").map((c,ci)=>elm("span",{class:correct[si].chars[ci]},[textelm(c)]))
                            );
                        } else if (seg.type == "Annotated") {
                            return elm("ruby",{class:"annotated"},[
                                elm("rb",{
                                        class:seg.reading.split("").map((c,ci)=>correct[si].chars[ci]).includes("Incorrect")?"Incorrect":"Correct"
                                    },
                                    [textelm(seg.base)]
                                ),
                                elm("rt",{},
                                    seg.reading.split("")
                                        .map((c,ci)=>elm("span",{class:correct[si].chars[ci]},[textelm(c)]))
                                ),
                            ]);
                        }
                    }),
                    elm("span",{class:"pendingSegment"},
                        (segment.type=="Annotated"?segment.reading:segment.text).slice(0,status.char_)
                            .split("")
                            .map((c,ci)=>elm("span",{class:correct[status.segment].chars[ci]},[textelm(c)]))
                    ),
                    elm("span",{class: "unconfirmed"},[textelm(status.unconfirmed.join(""))]),
                    elm("span",{class: "cursor"},[]),
                    elm("span",{class: "wrong"},[textelm(status.last_wrong_keydown!=null?status.last_wrong_keydown:"")]),
                ]
            )
        ]))
        const elements = document.querySelectorAll('.plain, .annotated');
        const lastElement = elements[elements.length-1];
        let anchor = (elements.length>0?lastElement:document.querySelector(".pendingSegment")).getBoundingClientRect();
        let target = 400;
        (document.querySelector(".typing_scroll") as HTMLDivElement).style.setProperty("--scroll-left",`${target-anchor.x}px`);
    }
    requestAnimationFrame(render);
}