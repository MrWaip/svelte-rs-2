import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let placeholder = $.prop($$props, "placeholder", 8);
	let fieldData = $.prop($$props, "fieldData", 8);
	$.init();
	var input = root();
	$.remove_input_defaults(input);
	$.template_effect(() => $.set_value(input, ($.deep_read_state(placeholder()), $.deep_read_state(fieldData()), $.untrack(() => placeholder() || fieldData().label))));
	$.append($$anchor, input);
	$.pop();
}
