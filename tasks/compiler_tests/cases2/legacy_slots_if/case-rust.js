import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<hr/> <!>`, 1);
var root = $.from_html(`<div><!> <!></div>`);
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	var div = root();
	var node = $.child(div);
	$.slot(node, $$props, "title", {}, null);
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			var fragment = root_1();
			var node_2 = $.sibling($.first_child(fragment), 2);
			$.slot(node_2, $$props, "description", {}, null);
			$.append($$anchor, fragment);
		};
		$.if(node_1, ($$render) => {
			if ($.untrack(() => $$slots.description)) $$render(consequent);
		});
	}
	$.reset(div);
	$.append($$anchor, div);
}
