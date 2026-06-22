import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>ok</p>`);
export default function App($$anchor, $$props) {
	class Counter {}
	var p = root();
	$.append($$anchor, p);
}
