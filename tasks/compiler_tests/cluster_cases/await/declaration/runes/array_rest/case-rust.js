import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
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
			var [a, ...rest] = $.get($$source);
			return {
				a,
				rest
			};
		});
		var a = $.derived(() => $.get($$value).a);
		var rest = $.derived(() => $.get($$value).rest);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(rest).length ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
