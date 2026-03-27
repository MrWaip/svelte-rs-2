import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = [1, 2], $$array = $.derived(() => $.to_array(tmp, 2)), x = $.state($.proxy($.get($$array)[0])), y = $.proxy($.get($$array)[1]);
	let tmp_1 = {
		a: 1,
		b: 2
	}, a = $.state($.proxy(tmp_1.a)), b = $.proxy(tmp_1.b);
	let tmp_2 = {}, c = $.state($.proxy($.fallback(tmp_2.c, 10))), d = $.proxy($.fallback(tmp_2.d, 20));
	let tmp_3 = { e: { f: 1 } }, f = $.proxy(tmp_3.e.f);
	let tmp_4 = {
		g: 1,
		h: 2,
		i: 3
	}, g = $.proxy(tmp_4.g), rest = $.proxy($.exclude_from_object(tmp_4, ["g"]));
	$.set(x, $.get(x) + 1);
	$.set(a, $.get(a) + 1);
	$.set(c, $.get(c) + 1);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""} ${y ?? ""} ${$.get(a) ?? ""} ${b ?? ""} ${$.get(c) ?? ""} ${d ?? ""} ${f ?? ""} ${g ?? ""}`));
	$.append($$anchor, p);
}
