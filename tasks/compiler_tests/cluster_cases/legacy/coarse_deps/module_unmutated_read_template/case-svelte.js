import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
let label = "hi";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	var p = root();
	p.textContent = "hi";
	$.append($$anchor, p);
}
