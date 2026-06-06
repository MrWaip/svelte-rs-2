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
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["value", "group"]);
	const binding_group = [];
	let value = $.prop($$props, "value", 8);
	let group = $.prop($$props, "group", 12);
	var input = root();
	$.attribute_effect(input, () => ({
		type: "radio",
		value: value(),
		...$$restProps
	}), void 0, void 0, void 0, void 0, true);
	$.bind_group(binding_group, [], input, () => {
		value();
		return group();
	}, group);
	$.append($$anchor, input);
}
