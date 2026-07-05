App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let src = $.tag_proxy($.proxy({
		"a-b": 1,
		"c d": 2
	}), "src");
	let ab = $.tag($.derived(() => src["a-b"]), "ab"), cd = $.tag($.derived(() => src["c d"]), "cd");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
