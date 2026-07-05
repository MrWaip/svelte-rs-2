import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve([1]));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var [a = 10, b = 20] = $.get($$source);
			return {
				a,
				b
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var b = $.derived(() => $.get($$value).b);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
