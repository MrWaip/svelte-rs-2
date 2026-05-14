import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<span>x</span>`);
var root_1 = $.from_html(`<!> <!>`, 1);
export default function App($$anchor, $$props) {
	let headerTag = $.prop($$props, "headerTag", 3, "div");
	let cond = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, headerTag, false, ($$element, $$anchor) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		$.snippet(node_1, () => $$props.header);
		var node_2 = $.sibling(node_1, 2);
		{
			var consequent = ($$anchor) => {
				var span = root_2();
				$.append($$anchor, span);
			};
			$.if(node_2, ($$render) => {
				if (cond) $$render(consequent);
			});
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
