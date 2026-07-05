import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let pairs = $.proxy({
		a: 1,
		b: 2
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { a: x, b: y } = pairs;
				return {
					x,
					y
				};
			});
			var button = root();
			var text = $.child(button);
			$.reset(button);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).x ?? ""}${$.get(computed_const).y ?? ""}`));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
