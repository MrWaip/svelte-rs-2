App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = $.tag_proxy($.proxy({
		a: 1,
		b: 2
	}), "data");
	let $$d = $.derived(() => data), a = $.tag($.derived(() => $.get($$d).a), "a"), b = $.tag($.derived(() => $.get($$d).b), "b");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""},${$.get(b) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
