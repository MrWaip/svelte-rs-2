import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, $$item) => {
		let item = () => $.exclude_from_object($.get($$item), []);
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, (item(), $.untrack(() => item().x))));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
