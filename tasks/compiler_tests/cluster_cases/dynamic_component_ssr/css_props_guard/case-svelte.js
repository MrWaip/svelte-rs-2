import * as $ from "svelte/internal/client";
import A from "./A.svelte";
import B from "./B.svelte";
var root = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	let value = 0;
	let Comp = $.derived(() => value % 2 === 0 ? A : B);
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({ "--prop": "red" }));
		$.component(node.lastChild, () => $.get(Comp), ($$anchor, Comp_1) => {
			Comp_1($$anchor, {});
		});
		$.reset(node);
	}
	$.append($$anchor, fragment);
}
