import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let extra = $.prop($$props, "extra", 19, () => ({}));
	var input = root();
	$.attribute_effect(input, () => ({ ...extra() }), void 0, void 0, void 0, void 0, true);
	$.append($$anchor, input);
}
