import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option> </option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.reset(select);
	select.value = select.__value = 42, $.select_option(select, 42);
	$.init_select(select);
	$.template_effect(($0, $1) => {
		$.set_text(text, $0);
		if (option_value !== (option_value = $1)) {
			option.__value = $1;
		}
	}, void 0, [() => Promise.resolve(42), () => Promise.resolve(42)]);
	$.append($$anchor, select);
}
