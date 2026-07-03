import * as $ from "svelte/internal/client";
import Tooltip from "./Tooltip.svelte";
import Icon from "./Icon.svelte";
var root = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	Tooltip($$anchor, { $$slots: { activator: ($$anchor, $$slotProps) => {
		var fragment_1 = root();
		var node = $.first_child(fragment_1);
		{
			$.css_props(node, () => ({ "--color": "red" }));
			Icon(node.lastChild, { slot: "activator" });
			$.reset(node);
		}
		$.append($$anchor, fragment_1);
	} } });
}
