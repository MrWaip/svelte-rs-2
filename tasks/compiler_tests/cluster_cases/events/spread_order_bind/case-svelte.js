import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let rest = $.prop($$props, "rest", 24, () => ({}));
	let value = $.prop($$props, "value", 12, "");
	var input = root();
	$.attribute_effect(input, () => ({ ...rest() }), void 0, void 0, void 0, void 0, true);
	$.bind_value(input, value);
	$.append($$anchor, input);
}
