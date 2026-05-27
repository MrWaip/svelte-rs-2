import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor) {
	let value = "a";
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "a";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	select.value = select.__value = value, $.select_option(select, value);
	$.init_select(select);
	$.append($$anchor, select);
}
