import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<img alt="x"/>`);
export default function App($$anchor, $$props) {
	let nw = $.prop($$props, "nw", 12);
	var img = root();
	$.bind_property("naturalWidth", "load", img, nw);
	$.append($$anchor, img);
}
