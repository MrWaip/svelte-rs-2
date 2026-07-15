App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_html(`<select></select>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root_1();
	$.add_svelte_meta(() => $.each(select, 21, () => $$props.items, $.index, ($$anchor, item) => {
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
	}), "each", App, 6, 1);
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
