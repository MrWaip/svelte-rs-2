App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag_proxy($.proxy([{ a: 1 }, [2, 3]]), "x");
	let $$array = $.tag($.derived(() => $.to_array(x, 2)), "[$derived iterable]"), $$array_1 = $.tag($.derived(() => $.to_array($.get($$array)[1], 2)), "[$derived iterable]"), a = $.tag($.derived(() => $.get($$array)[0].a), "a"), b = $.tag($.derived(() => $.get($$array_1)[0]), "b"), c = $.tag($.derived(() => $.get($$array_1)[1]), "c");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
