import * as $ from "svelte/internal/client";
var root = $.from_html(`<canvas></canvas>`);
export default function App($$anchor) {
	let el = $.state(void 0);
	var canvas = root();
	$.bind_this(canvas, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, canvas);
}
