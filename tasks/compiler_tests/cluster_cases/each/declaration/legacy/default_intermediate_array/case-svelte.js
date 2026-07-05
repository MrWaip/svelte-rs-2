import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [[[1, 2], 3]];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		var $$array_1 = $.derived(() => $.to_array($.fallback($.get($$array)[0], () => [8, 9], true), 2));
		let a = $.derived_safe_equal(() => $.get($$array_1)[0]);
		let b = $.derived_safe_equal(() => $.get($$array_1)[1]);
		let c = () => $.get($$array)[1];
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${c() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
