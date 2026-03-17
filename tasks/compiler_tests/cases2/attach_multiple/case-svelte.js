import * as $ from "svelte/internal/client";
import { tooltip, highlight } from "./actions.js";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.attach(div, () => tooltip);
	$.attach(div, () => highlight);
	$.append($$anchor, div);
}
