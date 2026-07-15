import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const handler = () => {};
	var $$exports = { handler };
	Child($$anchor, { onClose: handler });
	return $.pop($$exports);
}
