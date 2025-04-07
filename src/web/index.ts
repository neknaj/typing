import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { Model, Msg, Segment, TextOrientation, TypingCorrectnessSegment, TypingMetrics, TypingStatus } from "./model.js";

import initWasm, { start_gui } from './typing_lib.js';


async function run() {
    await initWasm();
    start_gui();
}

window.addEventListener("load",()=>{
    window.requestAnimationFrame(run);
})

document.querySelector("#screen").addEventListener("keydown",(e: KeyboardEvent)=>{
    if (e.key == "F11") {
        console.log(e)
        e.returnValue = false;
    }
})

document.addEventListener("keydown",(e: KeyboardEvent)=>{
    if (e.key == "F11") {
        console.log(e)
        e.returnValue = false;
    }
})