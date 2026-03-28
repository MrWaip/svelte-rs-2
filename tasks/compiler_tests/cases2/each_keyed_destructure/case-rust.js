import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p> </p>`);
var root_3 = $.from_html(`<p> </p>`);
var root = $.from_html(`<!> <!> <!>`, 1);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = root();
	var node = $.first_child(fragment);
	$.each(node, 17, items, ($$item) => id, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let id = () => $.get($$array)[0];
		let name = () => $.get($$array)[1];
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, name()));
		$.append($$anchor, p);
	});
	var node_1 = $.sibling(node, 2);
	$.each(node_1, 17, items, ($$item) => id, ($$anchor, $$item) => {
		let id = () => $.get($$item).id;
		let name = () => $.get($$item).name;
		var p_1 = root_2();
		var text_1 = $.child(p_1, true);
		$.reset(p_1);
		$.template_effect(() => $.set_text(text_1, name()));
		$.append($$anchor, p_1);
	});
	var node_2 = $.sibling(node_1, 2);
	$.each(node_2, 19, items, ($$item) => id, ($$anchor, $$item, idx) => {
		var $$array_1 = $.derived(() => $.to_array($.get($$item), 2));
		let id = () => $.get($$array_1)[0];
		let name = () => $.get($$array_1)[1];
		var p_2 = root_3();
		var text_2 = $.child(p_2);
		$.reset(p_2);
		$.template_effect(() => $.set_text(text_2, `${$.get(idx) ?? ""}: ${name() ?? ""}`));
		$.append($$anchor, p_2);
	});
	$.append($$anchor, fragment);
}
