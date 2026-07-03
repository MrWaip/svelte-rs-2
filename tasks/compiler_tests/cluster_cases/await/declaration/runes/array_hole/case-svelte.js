import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve([
		1,
		2,
		3
	]));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var [a, , c] = $.get($$source);
			return {
				a,
				c
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var c = $.derived(() => $.get($$value).c);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(c) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
