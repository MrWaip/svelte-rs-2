import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 19, items, ([id, name]) => id, ($$anchor, $$item, idx) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let id = () => $.get($$array)[0];
		let name = () => $.get($$array)[1];
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(idx) ?? ""}: ${name() ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
