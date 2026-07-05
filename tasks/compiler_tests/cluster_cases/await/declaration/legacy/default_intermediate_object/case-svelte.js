import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve({});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { p: { a } = {} } = $.get($$source);
			return { a };
		});
		var a = $.derived_safe_equal(() => $.get($$value).a);
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
