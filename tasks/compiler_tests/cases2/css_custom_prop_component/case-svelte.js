import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	let color = "red";
	let columns = 3;
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({
			"--color": color,
			"--columns": columns
		}));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.append($$anchor, fragment);
}
