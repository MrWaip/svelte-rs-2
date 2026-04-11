import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let checked = $.prop($$props, "checked", 15, false), disabled = $.prop($$props, "disabled", 3, false);
	var input = root();
	$.remove_input_defaults(input);
	$.template_effect(() => $.set_attribute(input, "disabled", disabled()));
	$.bind_checked(input, () => $.get(checked), ($$value) => $.set(checked, $$value));
	$.append($$anchor, input);
	$.pop();
}
