import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { layout_switch } from "./api.js";
import { Model, Msg, Segment, TextOrientation, TypingCorrectnessSegment, TypingMetrics, TypingStatus } from "./model.js";

import initWasm, { init_model, event_receive_keyboard, fetch_render_data, add_contents, typing_scroll } from './typing_lib.js';


async function init() {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    const layout = urlParams.get('layout')=="h"?"h":"v";
    await initWasm(); // Wasmモジュールの初期化
    // document.querySelector("html").dataset.layout = layout;
    // initlayout(
    //     document.querySelector("#layoutroot"),
    //     ["h",[5,3],[
    //         [layout=="h"?"v":"h",layout=="h"?[3,2]:[2,3],[
    //             ["c",layout=="h"?"main":"sub1"],
    //             ["c",layout=="h"?"sub1":"main"],
    //         ]],
    //         ["v",[2,5],[
    //             ["c","sub2"],
    //             ["c","sub3"],
    //         ]]
    //     ]],
    // );
    document.querySelector("#layoutroot").addProp({tabindex: 0}).Listen("keydown",(e: KeyboardEvent)=>{
        event_receive_keyboard(e.key);
    });
    await init_model(layout=="h"?"Horizontal":"Vertical" as TextOrientation);
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

    let now = performance.now();
    frameCount++;
    // Update the FPS value every 1000 milliseconds (1 second)
    if (now - lastFpsUpdate >= 1000) {
        fps = frameCount;
        frameCount = 0;
        lastFpsUpdate = now;
        document.querySelector("#overlay").Clear().Add(elm("p",{},[textelm("FPS: "),textelm(fps.toString())]))
    }
    if (data[0] == "Menu") {
        let selecting = data[1] as number;
        let menu = data[2] as string[];
        let layout = data[3] as TextOrientation;
        let [main,sub1,sub2,sub3] = layout_switch(layout);
        main.Clear();
        sub1.Clear();
        sub2.Clear();
        sub3.Clear();
        main.Add(elm("h1",{},[textelm("Neknaj Typing Game")])).Add(
            elm("ul", {}, menu.map(
                (content,i) => {
                    let e =elm("li", {}, [
                        textelm(content)
                    ]);
                    if (i==selecting) {
                        e.classList.add("selecting")
                    }
                    return e;
                }
            )))
    }
    if (data[0] == "TypingStart") {
        let title = data[1] as string;
        let layout = data[2] as TextOrientation;
        let [main,sub1,sub2,sub3] = layout_switch(layout);
        main.Clear();
        sub1.Clear();
        sub2.Clear();
        sub3.Clear();
        main.Add(elm("h1",{},[textelm(title)]))
            .Add(elm("p",{},[textelm("Press Space to start typing")]))
            .Add(elm("p",{},[textelm("Press Escape to cancel")]));
        let text_orientation = data[2] as TextOrientation;
        if (text_orientation=="Horizontal") {
            let w = main.getBoundingClientRect().width;
            typing_scroll(-w,-w);
        }
        else {
            let w = main.getBoundingClientRect().height;
            typing_scroll(-w,-w);
        }
    }
    if (data[0] == "Typing") {
        let title = data[1] as string;
        let segments = data[2] as Segment[];
        let correct = data[3] as TypingCorrectnessSegment[];
        let status = data[4] as TypingStatus;
        let text_orientation = data[5] as TextOrientation;
        let scroll = data[6] as number;
        let metrics = data[7] as TypingMetrics;
        let segment = segments[status.segment];
        console.log(metrics);
        let [main,sub1,sub2,sub3] = layout_switch(text_orientation);
        main.Clear();
        sub1.Clear();
        sub2.Clear();
        sub3.Clear();
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
        let anchor1 = (elements.length>0?lastElement:document.querySelector(".pendingSegment")).getBoundingClientRect();
        let anchor2 = document.querySelector(".cursor").getBoundingClientRect();
        if (text_orientation=="Horizontal") {
            let w = main.getBoundingClientRect().width;
            let target = w*0.3;
            typing_scroll((anchor1.x+anchor2.x*3)/4-target,-w);
            (document.querySelector(".typing_scroll") as HTMLDivElement).style.setProperty("--scroll",`${-scroll}px`);
        }
        else {
            let w = main.getBoundingClientRect().height;
            let target = w*0.3;
            typing_scroll((anchor1.y+anchor2.y*3)/4-target,-w);
            (document.querySelector(".typing_scroll") as HTMLDivElement).style.setProperty("--scroll",`${-scroll}px`);
        }
        sub3.Add(
            elm("div", { class: "metrics" }, [
                elm("table", {}, [
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Accuracy")]),
                        elm("td", {}, [textelm(`: ${(metrics.accuracy * 100).toFixed(2)}`)]),
                        elm("td", {}, [textelm(`%`)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Speed")]),
                        elm("td", {}, [textelm(`: ${metrics.speed.toFixed(2)}`)]),
                        elm("td", {}, [textelm(`chars/sec`)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Miss Count")]),
                        elm("td", {}, [textelm(`: ${metrics.miss_count}`)]),
                        elm("td", {}, [textelm(``)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Type Count")]),
                        elm("td", {}, [textelm(`: ${metrics.type_count}`)]),
                        elm("td", {}, [textelm(``)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Time")]),
                        elm("td", {}, [textelm(`: ${(metrics.total_time / 1000).toFixed(1)}`)]),
                        elm("td", {}, [textelm(`sec`)]),
                    ]),
                ]),
            ])
        );
    }
    if (data[0] == "Result") {
        let title = data[1] as string;
        let metrics = data[2] as TypingMetrics;
        let layout = data[3] as TextOrientation;
        let [main,sub1,sub2,sub3] = layout_switch(layout);
        main.Clear();
        sub1.Clear();
        sub2.Clear();
        sub3.Clear();
        main.Add(elm("h1",{},[textelm("Result")]))
            .Add(elm("h2",{},[textelm(title)]))
            .Add(elm("p",{},[textelm("Press Space to Restart typing")]))
            .Add(elm("p",{},[textelm("Press Escape to Back to Menu")]));
        sub3.Add(
            elm("div", { class: "metrics" }, [
                elm("table", {}, [
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Accuracy")]),
                        elm("td", {}, [textelm(`: ${(metrics.accuracy * 100).toFixed(2)}`)]),
                        elm("td", {}, [textelm(`%`)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Speed")]),
                        elm("td", {}, [textelm(`: ${metrics.speed.toFixed(2)}`)]),
                        elm("td", {}, [textelm(`chars/sec`)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Miss Count")]),
                        elm("td", {}, [textelm(`: ${metrics.miss_count}`)]),
                        elm("td", {}, [textelm(``)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Type Count")]),
                        elm("td", {}, [textelm(`: ${metrics.type_count}`)]),
                        elm("td", {}, [textelm(``)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Time")]),
                        elm("td", {}, [textelm(`: ${(metrics.total_time / 1000).toFixed(1)}`)]),
                        elm("td", {}, [textelm(`sec`)]),
                    ]),
                ]),
            ])
        );
    }
    if (data[0] == "Pause") {
        let title = data[1] as string;
        let metrics = data[2] as TypingMetrics;
        let layout = data[3] as TextOrientation;
        let [main,sub1,sub2,sub3] = layout_switch(layout);
        main.Clear();
        sub1.Clear();
        sub2.Clear();
        sub3.Clear();
        main.Add(elm("h1",{},[textelm("Pause")]))
            .Add(elm("h2",{},[textelm(title)]))
            .Add(elm("p",{},[textelm("Press Space to Resume typing")]))
            .Add(elm("p",{},[textelm("Press Escape to Finish")]));
        sub3.Add(
            elm("div", { class: "metrics" }, [
                elm("table", {}, [
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Accuracy")]),
                        elm("td", {}, [textelm(`: ${(metrics.accuracy * 100).toFixed(2)}`)]),
                        elm("td", {}, [textelm(`%`)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Speed")]),
                        elm("td", {}, [textelm(`: ${metrics.speed.toFixed(2)}`)]),
                        elm("td", {}, [textelm(`chars/sec`)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Miss Count")]),
                        elm("td", {}, [textelm(`: ${metrics.miss_count}`)]),
                        elm("td", {}, [textelm(``)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Type Count")]),
                        elm("td", {}, [textelm(`: ${metrics.type_count}`)]),
                        elm("td", {}, [textelm(``)]),
                    ]),
                    elm("tr", {}, [
                        elm("td", { class: "metric-label" }, [textelm("Time")]),
                        elm("td", {}, [textelm(`: ${(metrics.total_time / 1000).toFixed(1)}`)]),
                        elm("td", {}, [textelm(`sec`)]),
                    ]),
                ]),
            ])
        );
    }
    requestAnimationFrame(render);
}

function layout_switch(layout: TextOrientation) {
    let h = layout=="Horizontal";
    if (document.querySelector("html").dataset.layout == layout) {
        let main = (document.querySelector("#main") as HTMLDivElement);
        let sub1 = (document.querySelector("#sub1") as HTMLDivElement);
        let sub2 = (document.querySelector("#sub2") as HTMLDivElement);
        let sub3 = (document.querySelector("#sub3") as HTMLDivElement);
        return [main,sub1,sub2,sub3];
    }
    document.querySelector("html").dataset.layout = layout;
    initlayout(
        document.querySelector("#layoutroot"),
        ["h",[5,3],[
            [h?"v":"h",h?[3,2]:[2,3],[
                ["c",h?"main":"sub1"],
                ["c",h?"sub1":"main"],
            ]],
            ["v",[2,5],[
                ["c","sub2"],
                ["c","sub3"],
            ]]
        ]],
    );
    let main = (document.querySelector("#main") as HTMLDivElement);
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement);
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement);
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement);
    return [main,sub1,sub2,sub3];
}