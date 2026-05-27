import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve({
		"a-b": 1,
		"c d": 2
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { ab, cd } = $.get($$source);
			return {
				ab,
				cd
			};
		});
		var ab = $.derived_safe_equal(() => $.get($$value).ab);
		var cd = $.derived_safe_equal(() => $.get($$value).cd);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
