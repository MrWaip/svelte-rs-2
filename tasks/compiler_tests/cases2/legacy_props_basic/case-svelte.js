import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>click me</button>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false);
	$.init();
	var button = root();
	$.attribute_effect(button, () => ({
		...$$sanitized_props,
		class: ($.deep_read_state($$sanitized_props), $.untrack(() => $$sanitized_props.class ?? ""))
	}));
	$.append($$anchor, button);
	$.pop();
}
