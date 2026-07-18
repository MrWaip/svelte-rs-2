import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x">x</div><div class="a">a</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}
