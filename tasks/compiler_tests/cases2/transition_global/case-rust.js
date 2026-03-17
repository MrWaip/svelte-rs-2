import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.transition(7, div, () => fade);
	$.append($$anchor, div);
}
