import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let className = "btn";
	var $$exports = { class: className };
	var p = root();
	p.textContent = className;
	$.append($$anchor, p);
	return $.pop($$exports);
}
