import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let pairs = $.proxy({ outer: [{ inner: 1 }] });
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { outer: [{ inner }] } = pairs;
				return { inner };
			});
			var button = root_1();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, $.get(computed_const).inner));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
