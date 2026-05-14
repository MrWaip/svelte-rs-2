import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { x } = $$props.props;
				return { x };
			});
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(computed_const).x));
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
