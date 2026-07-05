import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let a = $.derived_safe_equal(() => $.fallback($.get($$item).a, 10));
		let b = $.derived_safe_equal(() => $.fallback($.get($$item).b, 20));
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
