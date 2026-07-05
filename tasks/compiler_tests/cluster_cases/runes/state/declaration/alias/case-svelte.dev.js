App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = {
		a: 1,
		b: 2
	}, x = $.tag_proxy($.proxy(tmp.a), "x"), y = $.tag_proxy($.proxy(tmp.b), "y");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${x ?? ""}${y ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
