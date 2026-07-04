import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let registry = undefined;
	const getRegistry = () => registry;
	registry = 1;
	var $$exports = {
		...$.legacy_api(),
		get getRegistry() {
			return getRegistry;
		}
	};
	$.bind_prop($$props, "getRegistry", getRegistry);
	return $.pop($$exports);
}
