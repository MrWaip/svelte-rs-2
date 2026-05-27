import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve({
		a: 1,
		b: 2
	}));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { x, y } = $.get($$source);
			return {
				x,
				y
			};
		});
		var x = $.derived(() => $.get($$value).x);
		var y = $.derived(() => $.get($$value).y);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(y) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
