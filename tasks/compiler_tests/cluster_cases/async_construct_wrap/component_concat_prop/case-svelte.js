import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	async function f() {
		return 1;
	}
	var fragment = root();
	var node = $.first_child(fragment);
	$.async(node, void 0, [() => f()], ($$anchor, $0) => {
		$.css_props(node, () => ({ "--c": "1px" }));
		Child(node.lastChild, { get a() {
			return `y${$.get($0) ?? ""}`;
		} });
		$.reset(node);
	});
	$.append($$anchor, fragment);
}
