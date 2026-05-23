import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["checked"]);
	let checked = $.prop($$props, "checked", 12, false);
	var input = root();
	$.attribute_effect(input, () => ({
		type: "checkbox",
		...$$restProps
	}), void 0, void 0, void 0, void 0, true);
	$.bind_checked(input, checked);
	$.append($$anchor, input);
}
