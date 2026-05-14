import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item)));
		let rest = () => $.get($$array).slice(0);
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, (rest(), $.untrack(() => rest()[0]))));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
