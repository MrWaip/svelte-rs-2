App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = [
		1,
		2,
		3
	], $$array = $.tag($.derived(() => $.to_array(tmp)), "[$state iterable]"), $$array_1 = $.tag($.derived(() => $.to_array($.get($$array).slice(1), 2)), "[$state iterable]"), a = $.tag_proxy($.proxy($.get($$array)[0]), "a"), b = $.tag_proxy($.proxy($.get($$array_1)[0]), "b"), c = $.tag_proxy($.proxy($.get($$array_1)[1]), "c");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${c ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
