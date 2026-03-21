import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p>No items</p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, items, $.index, ($$anchor, item) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.append($$anchor, p);
	}, ($$anchor) => {
		var p_1 = root_2();
		$.append($$anchor, p_1);
	});
	$.append($$anchor, fragment);
}
