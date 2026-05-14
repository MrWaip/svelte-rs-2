import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const items = [];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		const callback = $.derived(() => () => $.get(item).id);
		Child($$anchor, { callback: $.get(callback) });
	});
	$.append($$anchor, fragment);
}
