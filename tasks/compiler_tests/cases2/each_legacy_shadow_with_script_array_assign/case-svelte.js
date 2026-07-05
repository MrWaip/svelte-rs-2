import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
var root_1 = $.from_html(` <!>`, 1);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	let item = $.prop($$props, "item", 8);
	let target;
	function handle(e) {
		[target] = e;
	}
	$.next();
	var fragment = root_1();
	var text = $.first_child(fragment);
	text.nodeValue = `${handle} `;
	var node = $.sibling(text);
	$.each(node, 1, items, (item) => item.id, ($$anchor, item, $$index, $$array_1) => {
		var div = root();
		var text_1 = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text_1, ($.get(item), $.untrack(() => $.get(item).id))));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
