import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let selected = $.prop($$props, "selected", 15);
	var select = root();
	$.bind_select_value(select, selected);
	$.append($$anchor, select);
	$.pop();
}
