import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
var root_1 = $.from_html(`<select></select>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 9);
	var select = root_1();
	$.each(select, 5, items, $.index, ($$anchor, item) => {
		var option = root();
		var text = $.child(option, true);
		$.reset(option);
		var option_value = {};
		$.template_effect(() => {
			$.set_text(text, ($.get(item), $.untrack(() => $.get(item).text)));
			if (option_value !== (option_value = ($.get(item), $.untrack(() => $.get(item).value)))) {
				option.value = (option.__value = ($.get(item), $.untrack(() => $.get(item).value))) ?? "";
			}
		});
		$.append($$anchor, option);
	});
	$.reset(select);
	$.append($$anchor, select);
}
