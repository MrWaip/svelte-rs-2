import * as $ from "svelte/internal/client";
import Icon from "./Icon.svelte";
var root = $.from_html(`<span class="wrap svelte-1fxeua7"><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></span>`);
export default function App($$anchor, $$props) {
	let current = Icon;
	var span = root();
	var node = $.child(span);
	{
		$.css_props(node, () => ({ "--my-color": `var(--${$$props.color ?? ""})` }));
		$.component(node.lastChild, () => current, ($$anchor, $$component) => {
			$$component($$anchor, {});
		});
		$.reset(node);
	}
	$.reset(span);
	$.append($$anchor, span);
}
