import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const k = "z";
	let pairs = { z: 1 };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived_safe_equal(() => {
				const { [k]: v } = pairs;
				return { v };
			});
			var button = root();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, $.get(computed_const).v));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
