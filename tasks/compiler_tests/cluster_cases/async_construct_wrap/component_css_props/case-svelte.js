import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<button>inc</button> <svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, void 0, [() => delay($.get(x))], ($$anchor, $0) => {
		$.css_props(node, () => ({ "--color": $.get($0) }));
		Child(node.lastChild, {});
		$.reset(node);
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
