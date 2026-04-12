import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>has description</p>`);
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	const has_description = !!$$slots.description;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (has_description) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
