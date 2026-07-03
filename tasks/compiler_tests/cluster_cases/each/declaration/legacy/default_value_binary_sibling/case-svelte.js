import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [{}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let w = $.derived_safe_equal(() => $.fallback($.get($$item).w, 1));
		let x = $.derived_safe_equal(() => $.fallback($.get($$item).x, $.get(w) * 2));
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(w) ?? ""}${$.get(x) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
