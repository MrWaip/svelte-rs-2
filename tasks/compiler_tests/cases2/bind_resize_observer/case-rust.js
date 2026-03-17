import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <div></div>`, 1);
export default function App($$anchor) {
	let rect = $.state(void 0);
	let box_size = $.state(void 0);
	var fragment = root();
	var div = $.first_child(fragment);
	var div_1 = $.sibling(div, 2);
	$.bind_resize_observer(div, "contentRect", ($$value) => $.set(rect, $$value));
	$.bind_resize_observer(div_1, "contentBoxSize", ($$value) => $.set(box_size, $$value));
	$.append($$anchor, fragment);
}
