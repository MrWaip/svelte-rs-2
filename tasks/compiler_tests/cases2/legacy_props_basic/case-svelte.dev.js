import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>click me</button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false, App);
	let variant = $.prop($$props, "variant", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	$.attribute_effect(button, () => ({
		...$$sanitized_props,
		class: `variant-${variant() ?? ""} ${($.deep_read_state($$sanitized_props), $.untrack(() => $$sanitized_props.class ?? "")) ?? ""}`
	}));
	$.append($$anchor, button);
	return $.pop($$exports);
}
