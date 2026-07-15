import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let x = $.prop($$props, "x", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => $.untrack(() => Array(10).fill(null)), $.index, ($$anchor, _, i) => {
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${i}${x() ?? ""}`));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
	$.pop();
}
