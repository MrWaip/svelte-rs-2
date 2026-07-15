import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	function focus() {}
	var $$exports = { focus };
	Child($$anchor, { onClose: focus });
	return $.pop($$exports);
}
