import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 19, items, (item) => item.id, ($$anchor, item, i) => {
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(i) ?? ""}: ${$.get(item).name ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
