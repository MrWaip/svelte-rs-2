App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const handler = () => {};
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { props: { handler } }), "component", App, 6, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
