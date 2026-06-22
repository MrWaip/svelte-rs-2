import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<pre> </pre>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	let y = $.prop($$props, "y", 8);
	var pre = root();
	var text = $.child(pre, true);
	$.reset(pre);
	$.template_effect(($0) => $.set_text(text, $0), [() => ($.deep_read_state($$sanitized_props), $.untrack(() => JSON.stringify($$sanitized_props)))]);
	$.append($$anchor, pre);
}
