import * as $ from "svelte/internal/client";
var root = $.from_html(`<span><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></span>`);
export default function App($$anchor, $$props) {
	var span = root();
	var node = $.child(span);
	{
		$.css_props(node, () => ({ "--color": "red" }));
		$.component(node.lastChild, () => $$props.Icon, ($$anchor, $$component) => {
			$$component($$anchor, {});
		});
		$.reset(node);
	}
	$.reset(span);
	$.append($$anchor, span);
}
