App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = {
		p: [1, 2],
		q: [3, 4]
	}, $$array = $.tag($.derived(() => $.to_array(tmp.p, 2)), "[$state object]"), $$array_1 = $.tag($.derived(() => $.to_array(tmp.q, 2)), "[$state object]"), a = $.tag_proxy($.proxy($.get($$array)[0]), "a"), b = $.tag_proxy($.proxy($.get($$array)[1]), "b"), c = $.tag_proxy($.proxy($.get($$array_1)[0]), "c"), d = $.tag_proxy($.proxy($.get($$array_1)[1]), "d");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${c ?? ""}${d ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
