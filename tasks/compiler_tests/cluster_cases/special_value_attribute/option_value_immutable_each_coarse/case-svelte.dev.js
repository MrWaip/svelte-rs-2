import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[8, 2]]);
var root_1 = $.add_locations($.from_html(`<select></select>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 9);
	var $$exports = { ...$.legacy_api() };
	var select = root_1();
	$.add_svelte_meta(() => $.each(select, 5, items, $.index, ($$anchor, item) => {
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
	}), "each", App, 7, 1);
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
