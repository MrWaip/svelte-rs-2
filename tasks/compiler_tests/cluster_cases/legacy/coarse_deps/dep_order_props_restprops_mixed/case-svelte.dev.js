import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["a"]);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	$.legacy_pre_effect(() => ($.deep_read_state($$sanitized_props), $.deep_read_state(a()), $.deep_read_state($$restProps)), () => {
		$$sanitized_props, a(), $$restProps;
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
