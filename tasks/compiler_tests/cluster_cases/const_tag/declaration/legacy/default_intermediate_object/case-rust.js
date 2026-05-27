import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let pairs = {};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived_safe_equal(() => {
				const { p: { a } = {} } = pairs;
				return { a };
			});
			var button = root_1();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, $.get(computed_const).a));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
