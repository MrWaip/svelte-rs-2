import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let box_size = $.state(void 0);
	var div = root();
	$.bind_resize_observer(div, "devicePixelContentBoxSize", ($$value) => $.set(box_size, $$value));
	$.append($$anchor, div);
}
