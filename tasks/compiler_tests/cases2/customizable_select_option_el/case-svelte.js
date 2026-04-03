import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<div>text</div>`, 1);
var root = $.from_html(`<select><option><!></option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment = option_content();
		$.append(anchor, fragment);
	});
	$.reset(select);
	$.append($$anchor, select);
}
