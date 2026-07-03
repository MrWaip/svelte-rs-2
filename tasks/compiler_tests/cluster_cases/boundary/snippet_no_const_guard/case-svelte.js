import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>failed</p>`);
var root_2 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			var div = root_2();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $$props.n));
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
}
