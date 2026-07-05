import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		a: 1,
		b: 2
	}, a = $.prop($$props, "a", 28, () => tmp.a), b = $.prop($$props, "b", 28, () => tmp.b);
	function inc() {
		$.update_prop(a);
		$.update_prop(b);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.event("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
