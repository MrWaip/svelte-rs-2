import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let p = Promise.resolve({ k0: 1 });
	let num = $.mutable_source(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { [`k${$.get(num)}`]: v } = $.get($$source);
			return { v };
		});
		var v = $.derived_safe_equal(() => $.get($$value).v);
		var button_1 = root_1();
		var text = $.child(button_1, true);
		$.reset(button_1);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, button_1);
	});
	$.event("click", button, () => $.update(num));
	$.append($$anchor, fragment);
}
