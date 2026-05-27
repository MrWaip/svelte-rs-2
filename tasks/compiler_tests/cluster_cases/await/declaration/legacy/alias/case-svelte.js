import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve({
		a: 1,
		b: 2
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { a: x, b: y } = $.get($$source);
			return {
				x,
				y
			};
		});
		var x = $.derived_safe_equal(() => $.get($$value).x);
		var y = $.derived_safe_equal(() => $.get($$value).y);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(y) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
