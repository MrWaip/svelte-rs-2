import * as $ from "svelte/internal/client";
import { fly, fade } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.transition(1, div, () => fly, () => ({ y: 200 }));
	$.transition(2, div, () => fade);
	$.append($$anchor, div);
}
