import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<button>add</button> <!>`, 1);
export default function App($$anchor) {
	let items = $.proxy([[1]]);
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let a = () => $.get($$array)[0];
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, a()));
		$.append($$anchor, span);
	});
	$.delegated("click", button, () => items.push([items.length]));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
