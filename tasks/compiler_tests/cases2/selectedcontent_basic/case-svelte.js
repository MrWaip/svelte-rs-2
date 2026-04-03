import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<selectedcontent></selectedcontent>`, 1);
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var selectedcontent = $.first_child(fragment);
		$.selectedcontent(selectedcontent, ($$element) => selectedcontent = $$element);
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
