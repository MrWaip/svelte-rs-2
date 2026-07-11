import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	const handler = () => {};
	Child($$renderer, { onClose: handler });
	$.bind_props($$props, { handler });
}
