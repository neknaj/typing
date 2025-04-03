import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model, Msg, Segment, TextOrientation } from "./model.js";

import initWasm, { init_model, event_receive_keyboard, fetch_render_data } from './typing_lib.js';

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
}

window.addEventListener("load",init);

function render() {
    let data = JSON.parse(fetch_render_data());
    console.log("render")

    let main = (document.querySelector("#main") as HTMLDivElement).Clear();
    let sub1 = (document.querySelector("#sub1") as HTMLDivElement).Clear();
    let sub2 = (document.querySelector("#sub2") as HTMLDivElement).Clear();
    let sub3 = (document.querySelector("#sub3") as HTMLDivElement).Clear();
    if (false) {}
    else if (data[0]=="Menu") {
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
    requestAnimationFrame(render);
}