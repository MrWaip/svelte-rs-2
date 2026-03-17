import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	let enabled = true;
	var div = root();
	$.attach(div, () => enabled && tooltip);
	$.append($$anchor, div);
}
