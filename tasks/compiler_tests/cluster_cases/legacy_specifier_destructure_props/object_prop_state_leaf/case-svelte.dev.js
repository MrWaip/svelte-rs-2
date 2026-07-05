import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		a: 1,
		s: 2
	}, a = $.prop($$props, "a", 28, () => tmp.a), s = tmp.s;
	function inc() {
		$.update_prop(a);
		$.update(s);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$.get(s) ?? ""}`));
	$.event("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
