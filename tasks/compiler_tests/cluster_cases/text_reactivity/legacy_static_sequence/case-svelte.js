import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<pre></pre>`);
export default function App($$anchor, $$props) {
	let y = $.prop($$props, "y", 8);
	var pre = root();
	pre.textContent = (1, "");
	$.append($$anchor, pre);
}
