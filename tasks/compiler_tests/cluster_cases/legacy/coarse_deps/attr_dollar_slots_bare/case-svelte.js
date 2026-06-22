import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>a</p>`);
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $.prop($$props, "x", 8);
	var p = root();
	$.set_attribute(p, "hidden", $$slots);
	$.append($$anchor, p);
}
