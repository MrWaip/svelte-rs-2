import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	let list = $.prop($$props, "list", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => list() || [], $.index, ($$anchor, item, idx) => {
		Child($$anchor, { label: `ID (${idx + 1})` });
	});
	$.append($$anchor, fragment);
}
