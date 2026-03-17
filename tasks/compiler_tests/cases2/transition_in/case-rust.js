import * as $ from "svelte/internal/client";
import { fly } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.transition(1, div, () => fly);
	$.append($$anchor, div);
}
