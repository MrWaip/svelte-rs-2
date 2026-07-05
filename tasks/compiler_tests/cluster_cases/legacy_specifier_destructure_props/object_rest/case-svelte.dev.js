import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.prop($$props, "a", 28, () => tmp.a), rest = $.exclude_from_object(tmp, ["a"]);
	function inc() {
		$.update_prop(a);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `${a() ?? ""}${$0 ?? ""}`), [() => $.untrack(() => JSON.stringify(rest))]);
	$.event("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
