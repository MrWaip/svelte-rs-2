import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let registry = undefined;
	const getRegistry = () => registry;
	registry = 1;
	$.bind_props($$props, { getRegistry });
}
