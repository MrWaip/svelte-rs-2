import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<span>A</span>`, 1);
var root = $.from_html(`<select><option><!></option><option>B</option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment = option_content();
		$.append(anchor, fragment);
	});
	option.value = option.__value = "a";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	$.append($$anchor, select);
}
