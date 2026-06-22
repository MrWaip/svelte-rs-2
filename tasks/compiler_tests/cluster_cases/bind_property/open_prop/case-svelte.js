import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<details><summary>x</summary></details>`);
export default function App($$anchor, $$props) {
	let visible = $.prop($$props, "visible", 12);
	var details = root();
	$.bind_property("open", "toggle", details, visible, visible);
	$.append($$anchor, details);
}
