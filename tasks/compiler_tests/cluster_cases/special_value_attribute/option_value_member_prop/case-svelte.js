import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option> </option><option>Two</option></select>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let item = $.prop($$props, "item", 8);
	$.init();
	var select = root();
	var option = $.child(select);
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	$.template_effect(() => {
		$.set_text(text, ($.deep_read_state(item()), $.untrack(() => item().name)));
		if (option_value !== (option_value = ($.deep_read_state(item()), $.untrack(() => item().key)))) {
			option.value = (option.__value = ($.deep_read_state(item()), $.untrack(() => item().key))) ?? "";
		}
	});
	$.append($$anchor, select);
	$.pop();
}
