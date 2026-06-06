import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve({
		"a-b": 1,
		"c d": 2
	}));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { "a-b": ab, "c d": cd } = $.get($$source);
			return {
				ab,
				cd
			};
		});
		var ab = $.derived(() => $.get($$value).ab);
		var cd = $.derived(() => $.get($$value).cd);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
