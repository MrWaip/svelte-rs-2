import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false, App);
	const props = $.mutable_source();
	$.legacy_pre_effect(() => $.deep_read_state($$sanitized_props), () => {
		$.set(props, $$sanitized_props);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({ ...$.get(props) }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
