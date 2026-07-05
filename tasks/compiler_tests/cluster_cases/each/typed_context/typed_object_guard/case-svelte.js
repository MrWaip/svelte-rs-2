import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<button>add</button> <!>`, 1);
export default function App($$anchor) {
	let items = $.proxy([{ a: 1 }]);
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, a()));
		$.append($$anchor, span);
	});
	$.delegated("click", button, () => items.push({ a: items.length }));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
