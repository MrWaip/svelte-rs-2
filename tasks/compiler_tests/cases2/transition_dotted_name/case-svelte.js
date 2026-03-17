import * as $ from "svelte/internal/client";
import { custom } from "./transitions.js";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.transition(3, div, () => custom.fn);
	$.append($$anchor, div);
}
