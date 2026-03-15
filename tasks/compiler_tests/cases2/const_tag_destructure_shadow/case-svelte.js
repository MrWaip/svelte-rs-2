import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root = $.from_html(`<!> <p></p>`, 1);
export default function App($$anchor, $$props) {
	let computed_const = "shadow";
	var fragment = root();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $$props.items, $.index, ($$anchor, item) => {
		const computed_const_1 = $.derived(() => {
			const { x, y } = $.get(item);
			return {
				x,
				y
			};
		});
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(computed_const_1).x));
		$.append($$anchor, p);
	});
	var p_1 = $.sibling(node, 2);
	p_1.textContent = "shadow";
	$.append($$anchor, fragment);
}
