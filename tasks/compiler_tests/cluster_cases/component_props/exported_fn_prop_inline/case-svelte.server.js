import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	function focus() {}
	Child($$renderer, { onClose: focus });
	$.bind_props($$props, { focus });
}
