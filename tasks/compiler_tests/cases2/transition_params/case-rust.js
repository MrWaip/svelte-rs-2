import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.transition(3, div, () => fade, () => ({ duration: 300 }));
	$.append($$anchor, div);
}
