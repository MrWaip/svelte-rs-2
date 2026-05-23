import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false);
	const props = $.mutable_source();
	$.legacy_pre_effect(() => $.deep_read_state($$sanitized_props), () => {
		$.set(props, $$sanitized_props);
	});
	$.legacy_pre_effect_reset();
	var div = root();
	$.attribute_effect(div, () => ({ ...$.get(props) }));
	$.append($$anchor, div);
	$.pop();
}
