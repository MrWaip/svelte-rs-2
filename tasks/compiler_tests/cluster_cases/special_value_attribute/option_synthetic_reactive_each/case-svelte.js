import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<option> </option>`);
var root = $.from_html(`<select></select>`);
export default function App($$anchor, $$props) {
	let selected = $.prop($$props, "selected", 12);
	let items = $.prop($$props, "items", 8);
	var select = root();
	$.each(select, 5, items, $.index, ($$anchor, item) => {
		var option = root_1();
		var text = $.child(option, true);
		$.reset(option);
		var option_value = {};
		$.template_effect(() => {
			$.set_text(text, $.get(item));
			if (option_value !== (option_value = $.get(item))) {
				option.__value = $.get(item);
			}
		});
		$.append($$anchor, option);
	});
	$.reset(select);
	$.bind_select_value(select, selected);
	$.append($$anchor, select);
}
