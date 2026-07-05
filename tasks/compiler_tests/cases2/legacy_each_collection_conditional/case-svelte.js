import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let cond = $.prop($$props, "cond", 8, true);
	let a = $.prop($$props, "a", 24, () => [1]);
	let b = $.prop($$props, "b", 24, () => [2]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => cond() ? a() : b(), $.index, ($$anchor, item) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
