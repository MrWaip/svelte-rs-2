import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var p = root();
	p.textContent = $.untrack(() => "hello".toUpperCase());
	$.append($$anchor, p);
	$.pop();
}
