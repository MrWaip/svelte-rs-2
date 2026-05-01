import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1></h1> `, 1);
export default function App($$anchor) {
	let name = "world";
	var fragment = root();
	var h1 = $.first_child(fragment);
	h1.textContent = "Hello world!";
	var text = $.sibling(h1);
	$.template_effect(($0) => $.set_text(text, ` ${$0 ?? ""}`), void 0, [fetch]);
	$.append($$anchor, fragment);
}
