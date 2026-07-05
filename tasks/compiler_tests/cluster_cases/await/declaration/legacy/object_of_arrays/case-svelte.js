import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve({
		p: [1, 2],
		q: [3, 4]
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { p: [a, b], q: [c, d] } = $.get($$source);
			return {
				a,
				b,
				c,
				d
			};
		});
		var a = $.derived_safe_equal(() => $.get($$value).a);
		var b = $.derived_safe_equal(() => $.get($$value).b);
		var c = $.derived_safe_equal(() => $.get($$value).c);
		var d = $.derived_safe_equal(() => $.get($$value).d);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
