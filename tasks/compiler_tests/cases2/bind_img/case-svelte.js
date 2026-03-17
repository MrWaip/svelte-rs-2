import * as $ from "svelte/internal/client";
var root = $.from_html(`<img/>`);
export default function App($$anchor) {
	let naturalWidth = $.state(0);
	let naturalHeight = $.state(0);
	var img = root();
	$.bind_property("naturalWidth", "load", img, ($$value) => $.set(naturalWidth, $$value));
	$.bind_property("naturalHeight", "load", img, ($$value) => $.set(naturalHeight, $$value));
	$.append($$anchor, img);
}
