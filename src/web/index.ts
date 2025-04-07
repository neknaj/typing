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
});