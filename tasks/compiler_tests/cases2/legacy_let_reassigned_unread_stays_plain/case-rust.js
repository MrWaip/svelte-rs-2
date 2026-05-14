import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let registry = undefined;
	const getRegistry = () => registry;
	registry = 1;
	var $$exports = { getRegistry };
	$.bind_prop($$props, "getRegistry", getRegistry);
	return $.pop($$exports);
}
