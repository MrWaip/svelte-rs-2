import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<div> </div>`, 1);
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor, $$props) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var div = $.first_child(fragment);
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $$props.label));
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
