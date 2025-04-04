import { elm, textelm } from './cdom.js';
import { initlayout } from "./layout.js";
import { layout_switch } from "./api.js";
import { Model, Msg, Segment, TextOrientation, TypingCorrectnessSegment, TypingMetrics, TypingStatus } from "./model.js";

import initWasm, { start_gui ,init_model, event_receive_keyboard, fetch_render_data, add_contents, typing_scroll } from './typing_lib.js';


async function run() {
    await initWasm();
    start_gui();
}

run();