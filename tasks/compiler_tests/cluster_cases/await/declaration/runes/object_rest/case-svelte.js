import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve({
		a: 1,
		b: 2,
		c: 3
	}));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { a, ...rest } = $.get($$source);
			return {
				a,
				rest
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var rest = $.derived(() => $.get($$value).rest);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(rest).b ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
