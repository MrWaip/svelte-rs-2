import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>click me</button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["variant", "size"]);
	$.push($$props, false, App);
	let variant = $.prop($$props, "variant", 8, "filled");
	let size = $.prop($$props, "size", 8, "md");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.attribute_effect(button, () => ({
		...$$restProps,
		class: `variant-${variant() ?? ""} size-${size() ?? ""}`
	}));
	$.append($$anchor, button);
	return $.pop($$exports);
}
