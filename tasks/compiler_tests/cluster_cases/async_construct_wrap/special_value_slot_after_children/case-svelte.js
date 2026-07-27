import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>--Please choose an option--</option><option> </option><option>cat</option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.sibling($.child(select));
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.next();
	$.reset(select);
	var select_value;
	$.init_select(select);
	$.template_effect(($0, $1, $2) => {
		$.set_text(text, $0);
		if (option_value !== (option_value = $1)) {
			option.__value = $1;
		}
		if (select_value !== (select_value = $2)) {
			select.value = (select.__value = $2) ?? "", $.select_option(select, $2);
		}
	}, void 0, [
		() => Promise.resolve("dog"),
		() => Promise.resolve("dog"),
		() => Promise.resolve("dog")
	]);
	$.append($$anchor, select);
}
