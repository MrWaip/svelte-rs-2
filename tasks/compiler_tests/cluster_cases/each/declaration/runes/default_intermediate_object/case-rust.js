import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let a = $.derived_safe_equal(() => $.fallback($.get($$item).p, () => ({}), true).a);
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
