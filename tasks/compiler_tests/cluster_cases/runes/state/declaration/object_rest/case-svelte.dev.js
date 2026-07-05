App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.tag_proxy($.proxy(tmp.a), "a"), rest = $.tag_proxy($.proxy($.exclude_from_object(tmp, ["a"])), "rest");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${rest.b ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
