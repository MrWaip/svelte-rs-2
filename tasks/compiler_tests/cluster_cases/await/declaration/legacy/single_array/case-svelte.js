import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve([1, 2]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var [a, b] = $.get($$source);
			return {
				a,
				b
			};
		});
		var a = $.derived_safe_equal(() => $.get($$value).a);
		var b = $.derived_safe_equal(() => $.get($$value).b);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
