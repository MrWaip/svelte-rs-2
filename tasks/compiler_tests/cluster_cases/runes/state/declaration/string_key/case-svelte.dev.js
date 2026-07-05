App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = $.tag_proxy($.proxy(tmp["a-b"]), "ab"), cd = $.tag_proxy($.proxy(tmp["c d"]), "cd");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${ab ?? ""}${cd ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
