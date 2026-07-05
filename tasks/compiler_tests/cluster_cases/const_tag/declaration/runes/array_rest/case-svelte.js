import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let pairs = $.proxy([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const [a, ...rest] = pairs;
				return {
					a,
					rest
				};
			});
			var button = root();
			var text = $.child(button);
			$.reset(button);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).a ?? ""}${$.get(computed_const).rest.length ?? ""}`));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
