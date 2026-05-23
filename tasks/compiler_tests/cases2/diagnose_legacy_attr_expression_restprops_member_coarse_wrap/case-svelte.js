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
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["name"]);
	$.push($$props, false);
	let name = $.prop($$props, "name", 8, "n");
	$.init();
	var div = root();
	$.template_effect(() => $.set_attribute(div, "id", ($.deep_read_state($$restProps), $.deep_read_state(name()), $.untrack(() => $$restProps.id || name()))));
	$.append($$anchor, div);
	$.pop();
}
