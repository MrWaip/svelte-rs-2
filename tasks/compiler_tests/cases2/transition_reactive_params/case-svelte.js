import * as $ from "svelte/internal/client";
import { fly } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	let y = 200;
	var div = root();
	$.transition(3, div, () => fly, () => ({ y }));
	$.append($$anchor, div);
}
