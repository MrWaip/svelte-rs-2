import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
var root_1 = $.from_html(`<select></select>`);
export default function App($$anchor, $$props) {
	var select = root_1();
	$.each(select, 21, () => $$props.items, $.index, ($$anchor, item) => {
		var option = root();
		var text = $.child(option, true);
		$.reset(option);
		var option_value = {};
		$.template_effect(() => {
			$.set_text(text, $.get(item).text);
			if (option_value !== (option_value = $.get(item).value)) {
				option.value = (option.__value = $.get(item).value) ?? "";
			}
		});
		$.append($$anchor, option);
	});
	$.reset(select);
	$.append($$anchor, select);
}
