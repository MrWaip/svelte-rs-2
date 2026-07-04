import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		a: 1,
		b: 2
	}, x = $.prop($$props, "x", 24, () => tmp.a), y = $.prop($$props, "y", 24, () => tmp.b);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${x() ?? ""}${y() ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
