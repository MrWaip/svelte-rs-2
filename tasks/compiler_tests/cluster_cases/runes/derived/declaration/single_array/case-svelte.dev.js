App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag_proxy($.proxy([1, 2]), "x");
	let $$array = $.tag($.derived(() => $.to_array(x, 2)), "[$derived iterable]"), a = $.tag($.derived(() => $.get($$array)[0]), "a"), b = $.tag($.derived(() => $.get($$array)[1]), "b");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
