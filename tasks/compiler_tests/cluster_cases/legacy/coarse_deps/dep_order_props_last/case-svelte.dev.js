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
	$.push($$props, false, App);
	let id = $.prop($$props, "id", 8);
	let callback = $.prop($$props, "callback", 8);
	$.legacy_pre_effect(() => ($.deep_read_state(callback()), $.deep_read_state(id()), $.deep_read_state($$sanitized_props)), () => {
		callback()(id()), $$sanitized_props;
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	return $.pop($$exports);
}
