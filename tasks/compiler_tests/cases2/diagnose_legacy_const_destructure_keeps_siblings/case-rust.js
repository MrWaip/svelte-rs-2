import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let src = $.prop($$props, "src", 8);
	const tmp = src(), a = tmp.a, b = $.mutable_source(tmp.b), c = tmp.c;
	const tmp_1 = src().list, $$array = $.derived(() => $.to_array(tmp_1, 2)), d = $.get($$array)[0], e = $.mutable_source($.get($$array)[1]);
	const tmp_2 = src().nested, f = tmp_2.f, h = $.mutable_source(tmp_2.g.h), j = tmp_2.g.i;
	const tmp_3 = src().mixed, $$array_1 = $.derived(() => $.to_array(tmp_3.k, 2)), l = $.mutable_source($.get($$array_1)[0]), m = $.get($$array_1)[1];
	const tmp_4 = src().arrobj, $$array_2 = $.derived(() => $.to_array(tmp_4, 2)), n = $.get($$array_2)[0], o = $.mutable_source($.get($$array_2)[1].o), p = $.get($$array_2)[1].p;
	const tmp_5 = src().renamed, r = $.mutable_source(tmp_5.q), t = tmp_5.s;
	function mutate() {
		$.mutate(b, $.get(b).x = 1);
		$.mutate(e, $.get(e).x = 2);
		$.mutate(h, $.get(h).x = 3);
		$.mutate(l, $.get(l).x = 4);
		$.mutate(o, $.get(o).x = 5);
		$.mutate(r, $.get(r).x = 6);
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""} ${($.get(b), $.untrack(() => $.get(b).x)) ?? ""} ${c ?? ""}
    ${d ?? ""} ${($.get(e), $.untrack(() => $.get(e).x)) ?? ""}
    ${f ?? ""} ${($.get(h), $.untrack(() => $.get(h).x)) ?? ""} ${j ?? ""}
    ${($.get(l), $.untrack(() => $.get(l).x)) ?? ""} ${m ?? ""}
    ${n ?? ""} ${($.get(o), $.untrack(() => $.get(o).x)) ?? ""} ${p ?? ""}
    ${($.get(r), $.untrack(() => $.get(r).x)) ?? ""} ${t ?? ""}`));
	$.event("click", button, mutate);
	$.append($$anchor, button);
	$.pop();
}
