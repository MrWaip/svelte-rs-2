App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag_proxy($.proxy({
		p: { a: 1 },
		q: { b: 2 }
	}), "x");
	let a = $.tag($.derived(() => x.p.a), "a"), b = $.tag($.derived(() => x.q.b), "b");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
