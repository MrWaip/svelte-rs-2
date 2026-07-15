App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const DEFAULTS = { a: 1 };
	let config = $.prop($$props, "config", 27, () => $.tag_proxy($.proxy(DEFAULTS), "config"));
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, config().a));
	$.append($$anchor, button);
	return $.pop($$exports);
}
