App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = {
		p: { a: 1 },
		q: { b: 2 }
	}, a = $.tag_proxy($.proxy(tmp.p.a), "a"), b = $.tag_proxy($.proxy(tmp.q.b), "b");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
