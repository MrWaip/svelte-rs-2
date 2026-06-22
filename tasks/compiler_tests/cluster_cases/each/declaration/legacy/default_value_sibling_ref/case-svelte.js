import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [{}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let width = $.derived_safe_equal(() => $.fallback($.get($$item).width, 10));
		let area = $.derived_safe_equal(() => $.fallback($.get($$item).area, () => Math.max($.get(width), 0), true));
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(width) ?? ""}${$.get(area) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
