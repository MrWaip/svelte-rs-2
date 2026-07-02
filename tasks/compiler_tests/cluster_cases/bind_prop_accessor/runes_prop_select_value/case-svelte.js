import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 7, "a");
	var select = root();
	$.bind_select_value(select, value);
	$.append($$anchor, select);
}
