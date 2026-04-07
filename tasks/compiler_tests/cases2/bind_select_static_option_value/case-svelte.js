import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>A</option><option>B</option></select>`);
export default function App($$anchor) {
	let selected = $.state("a");
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "a";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	$.bind_select_value(select, () => $.get(selected), ($$value) => $.set(selected, $$value));
	$.append($$anchor, select);
}
