import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let title = $$props["title"];
	Child($$renderer, { title });
	$.bind_props($$props, { title });
}
