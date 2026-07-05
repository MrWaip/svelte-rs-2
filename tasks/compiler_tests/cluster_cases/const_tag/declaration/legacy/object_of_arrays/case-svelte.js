import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let pairs = {
		p: [1, 2],
		q: [3, 4]
	};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived_safe_equal(() => {
				const { p: [a, b], q: [c, d] } = pairs;
				return {
					a,
					b,
					c,
					d
				};
			});
			var button = root();
			var text = $.child(button);
			$.reset(button);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).a ?? ""}${$.get(computed_const).b ?? ""}${$.get(computed_const).c ?? ""}${$.get(computed_const).d ?? ""}`));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
