import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option disabled="">Select an option</option></select>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 12);
	var select = root();
	var option = $.child(select);
	option.value = (option.__value = null) ?? "";
	$.reset(select);
	$.bind_select_value(select, foo);
	$.append($$anchor, select);
}
