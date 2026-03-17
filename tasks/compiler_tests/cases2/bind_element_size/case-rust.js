import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <div></div>`, 1);
export default function App($$anchor) {
	let w = $.state(0);
	let h = $.state(0);
	var fragment = root();
	var div = $.first_child(fragment);
	var div_1 = $.sibling(div, 2);
	$.bind_element_size(div, "clientWidth", ($$value) => $.set(w, $$value));
	$.bind_element_size(div, "clientHeight", ($$value) => $.set(h, $$value));
	$.bind_element_size(div_1, "offsetWidth", ($$value) => $.set(w, $$value));
	$.bind_element_size(div_1, "offsetHeight", ($$value) => $.set(h, $$value));
	$.append($$anchor, fragment);
}
