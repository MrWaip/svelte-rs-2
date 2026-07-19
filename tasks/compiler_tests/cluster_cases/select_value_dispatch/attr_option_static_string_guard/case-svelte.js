import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>Dog</option><option>Cat</option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "dog";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "cat";
	$.reset(select);
	$.append($$anchor, select);
}
