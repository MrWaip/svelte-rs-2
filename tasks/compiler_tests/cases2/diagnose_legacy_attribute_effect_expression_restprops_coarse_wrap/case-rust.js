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
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["name", "checked"]);
	$.push($$props, false);
	let name = $.prop($$props, "name", 8, "");
	let checked = $.prop($$props, "checked", 12, false);
	$.init();
	var input = root();
	$.attribute_effect(input, () => ({
		type: "checkbox",
		id: ($.deep_read_state($$restProps), $.deep_read_state(name()), $.untrack(() => $$restProps.id || name())),
		...$$restProps
	}), void 0, void 0, void 0, void 0, true);
	$.bind_checked(input, checked);
	$.append($$anchor, input);
	$.pop();
}
