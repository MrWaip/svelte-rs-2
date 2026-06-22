import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor, $$props) {
	let selected = $.prop($$props, "selected", 12);
	var select = root();
	$.bind_select_value(select, selected);
	$.append($$anchor, select);
}
