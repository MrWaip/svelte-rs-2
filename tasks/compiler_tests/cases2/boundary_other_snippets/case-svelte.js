import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span>helper text</span>`);
var root_2 = $.from_html(`<p> </p> <!>`, 1);
var root_3 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const helper = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		const failed = ($$anchor, error = $.noop) => {
			var fragment_1 = root_2();
			var p = $.first_child(fragment_1);
			var text = $.child(p, true);
			$.reset(p);
			var node_1 = $.sibling(p, 2);
			helper(node_1);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, fragment_1);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			var p_1 = root_3();
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
}
