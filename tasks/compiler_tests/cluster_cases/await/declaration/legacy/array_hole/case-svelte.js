import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var [a, , c] = $.get($$source);
			return {
				a,
				c
			};
		});
		var a = $.derived_safe_equal(() => $.get($$value).a);
		var c = $.derived_safe_equal(() => $.get($$value).c);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(c) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
