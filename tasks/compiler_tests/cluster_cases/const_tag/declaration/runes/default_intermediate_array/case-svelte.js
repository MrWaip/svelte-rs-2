import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let pairs = $.proxy([[1, 2], 3]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const [[a, b] = [8, 9], c] = pairs;
				return {
					a,
					b,
					c
				};
			});
			var button = root();
			var text = $.child(button);
			$.reset(button);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).a ?? ""}${$.get(computed_const).b ?? ""}${$.get(computed_const).c ?? ""}`));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
