import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, items, $.index, ($$anchor, $$item) => {
		let name = () => $.get($$item).name;
		let value = $.derived_safe_equal(() => $.fallback($.get($$item).value, "N/A"));
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""}: ${$.get(value) ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
