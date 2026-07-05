import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { name, label = "fallback" } = $$props.item;
				return {
					name,
					label
				};
			});
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).name ?? ""} ${$.get(computed_const).label ?? ""}`));
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($$props.item) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
