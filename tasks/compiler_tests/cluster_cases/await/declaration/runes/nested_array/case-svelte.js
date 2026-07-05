import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve([[1, 2], [3, 4]]));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var [[a, b], [c, d]] = $.get($$source);
			return {
				a,
				b,
				c,
				d
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var b = $.derived(() => $.get($$value).b);
		var c = $.derived(() => $.get($$value).c);
		var d = $.derived(() => $.get($$value).d);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
