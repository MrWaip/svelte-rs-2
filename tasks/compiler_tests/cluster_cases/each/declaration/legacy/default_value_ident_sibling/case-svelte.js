import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let array = [{
		a: 1,
		c: 2
	}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => array, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		let b = $.derived_safe_equal(() => $.fallback($.get($$item).b, c));
		let c = () => $.get($$item).c;
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${$.get(b) ?? ""}${c() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
