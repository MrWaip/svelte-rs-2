App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let coords = $.tag_proxy($.proxy({
		x: 0,
		y: 0
	}), "coords");
	let x = $.tag($.derived(() => coords.x), "x"), y = $.tag($.derived(() => coords.y), "y");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""},${$.get(y) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
