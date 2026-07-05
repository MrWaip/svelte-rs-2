App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = [[1, 2], [3, 4]], $$array = $.tag($.derived(() => $.to_array(tmp, 2)), "[$state iterable]"), $$array_1 = $.tag($.derived(() => $.to_array($.get($$array)[0], 2)), "[$state iterable]"), $$array_2 = $.tag($.derived(() => $.to_array($.get($$array)[1], 2)), "[$state iterable]"), a = $.tag_proxy($.proxy($.get($$array_1)[0]), "a"), b = $.tag_proxy($.proxy($.get($$array_1)[1]), "b"), c = $.tag_proxy($.proxy($.get($$array_2)[0]), "c"), d = $.tag_proxy($.proxy($.get($$array_2)[1]), "d");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${c ?? ""}${d ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
