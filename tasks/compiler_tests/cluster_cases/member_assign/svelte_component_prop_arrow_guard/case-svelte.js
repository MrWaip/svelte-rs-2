import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	let C = $.proxy(Child);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => C, ($$anchor, $$component) => {
		$$component($$anchor, { onChange: (v) => obj.x = src });
	});
	$.append($$anchor, fragment);
}
