App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = [1, 2], $$array = $.tag($.derived(() => $.to_array(tmp, 2)), "[$state iterable]"), x = $.tag($.state($.proxy($.get($$array)[0])), "x"), y = $.tag_proxy($.proxy($.get($$array)[1]), "y");
	let tmp_1 = {
		a: 1,
		b: 2
	}, a = $.tag($.state($.proxy(tmp_1.a)), "a"), b = $.tag_proxy($.proxy(tmp_1.b), "b");
	let tmp_2 = {}, c = $.tag($.state($.proxy($.fallback(tmp_2.c, 10))), "c"), d = $.tag_proxy($.proxy($.fallback(tmp_2.d, 20)), "d");
	let tmp_3 = { e: { f: 1 } }, f = $.tag_proxy($.proxy(tmp_3.e.f), "f");
	let tmp_4 = {
		g: 1,
		h: 2,
		i: 3
	}, g = $.tag_proxy($.proxy(tmp_4.g), "g"), rest = $.tag_proxy($.proxy($.exclude_from_object(tmp_4, ["g"])), "rest");
	$.set(x, $.get(x) + 1);
	$.set(a, $.get(a) + 1);
	$.set(c, $.get(c) + 1);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""} ${y ?? ""} ${$.get(a) ?? ""} ${b ?? ""} ${$.get(c) ?? ""} ${d ?? ""} ${f ?? ""} ${g ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
