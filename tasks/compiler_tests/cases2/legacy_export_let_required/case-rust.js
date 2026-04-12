import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	var p = root();
	p.textContent = $$props.foo;
	$.append($$anchor, p);
}
