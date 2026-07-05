import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let kind = $.prop($$props, "kind", 8, "a");
	let label = $.prop($$props, "label", 24, () => $.strict_equals(kind(), "a") ? "first" : "second");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, label()));
	$.append($$anchor, div);
	return $.pop($$exports);
}
