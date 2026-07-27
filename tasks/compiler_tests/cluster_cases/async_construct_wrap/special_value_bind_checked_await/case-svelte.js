import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	let g = $.state(void 0);
	var input = root();
	$.remove_input_defaults(input);
	var input_value;
	$.template_effect(($0) => {
		if (input_value !== (input_value = $0)) {
			input.value = (input.__value = $0) ?? "";
		}
	}, void 0, [() => $$props.a]);
	$.bind_checked(input, () => $.get(g), ($$value) => $.set(g, $$value));
	$.append($$anchor, input);
}
