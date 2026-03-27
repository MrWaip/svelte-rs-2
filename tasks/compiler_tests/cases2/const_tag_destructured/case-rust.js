import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $$props.items, $.index, ($$anchor, item) => {
		const computed_const = $.derived(() => {
			const { x, y } = $.get(item);
			return {
				x,
				y
			};
		});
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(computed_const).x));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
