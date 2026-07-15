App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const handler = () => {};
	var $$exports = {
		...$.legacy_api(),
		get handler() {
			return handler;
		}
	};
	$.add_svelte_meta(() => Child($$anchor, { onClose: handler }), "component", App, 6, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
