import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><input type="checkbox"/></div>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["checked"]);
	let checked = $.prop($$props, "checked", 12, false);
	function k() {}
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	var input = $.child(div);
	$.remove_input_defaults(input);
	$.reset(div);
	$.bind_checked(input, checked);
	$.event("click", div, k);
	$.append($$anchor, div);
}
