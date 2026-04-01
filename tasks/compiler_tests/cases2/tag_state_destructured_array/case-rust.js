App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = [1, 2], $$array = $.tag($.derived(() => $.to_array(tmp, 2)), "[$state iterable]"), a = $.tag($.state($.proxy($.get($$array)[0])), "a"), b = $.tag($.state($.proxy($.get($$array)[1])), "b");
	$.set(a, 10);
	$.set(b, 20);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${$.get(b) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
