import * as $ from "svelte/internal/client";
var root = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	const W = 120;
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({ "--cellWidth": "120px" }));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.append($$anchor, fragment);
}
