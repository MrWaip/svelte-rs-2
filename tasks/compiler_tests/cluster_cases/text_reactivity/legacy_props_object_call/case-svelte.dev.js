import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<pre> </pre>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false, App);
	let y = $.prop($$props, "y", 8);
	var $$exports = { ...$.legacy_api() };
	var pre = root();
	var text = $.child(pre, true);
	$.reset(pre);
	$.template_effect(($0) => $.set_text(text, $0), [() => ($.deep_read_state($$sanitized_props), $.untrack(() => JSON.stringify($$sanitized_props)))]);
	$.append($$anchor, pre);
	return $.pop($$exports);
}
