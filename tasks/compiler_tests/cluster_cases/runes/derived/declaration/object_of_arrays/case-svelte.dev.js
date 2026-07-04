App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag_proxy($.proxy({
		p: [1, 2],
		q: [3, 4]
	}), "x");
	let $$array = $.tag($.derived(() => $.to_array(x.p, 2)), "[$derived object]"), $$array_1 = $.tag($.derived(() => $.to_array(x.q, 2)), "[$derived object]"), a = $.tag($.derived(() => $.get($$array)[0]), "a"), b = $.tag($.derived(() => $.get($$array)[1]), "b"), c = $.tag($.derived(() => $.get($$array_1)[0]), "c"), d = $.tag($.derived(() => $.get($$array_1)[1]), "d");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
