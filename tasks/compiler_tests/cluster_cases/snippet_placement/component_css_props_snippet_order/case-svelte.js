import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<div> </div>`);
var root_1 = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		const element = ($$anchor, $$arg0) => {
			let idx = () => ($$arg0?.()).idx;
			var div = root();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, idx()));
			$.append($$anchor, div);
		};
		$.css_props(node, () => ({ "--my-var": "baseline" }));
		Child(node.lastChild, {
			get value() {
				return $$props.data;
			},
			element,
			$$slots: { element: true }
		});
		$.reset(node);
	}
	$.append($$anchor, fragment);
}
