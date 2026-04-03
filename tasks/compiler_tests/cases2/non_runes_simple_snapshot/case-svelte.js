import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>legacy</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
