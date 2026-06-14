import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var option = root();
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.template_effect(() => {
		$.set_text(text, foo());
		if (option_value !== (option_value = foo())) {
			option.value = (option.__value = foo()) ?? "";
		}
	});
	$.append($$anchor, option);
}
