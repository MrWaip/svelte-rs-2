import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.prop($$props, "a", 24, () => tmp.a), rest = $.prop($$props, "rest", 24, () => $.exclude_from_object(tmp, ["a"]));
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${($.deep_read_state(rest()), $.untrack(() => rest().b)) ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
