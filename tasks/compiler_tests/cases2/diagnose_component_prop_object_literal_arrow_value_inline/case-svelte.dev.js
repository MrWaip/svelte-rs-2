App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
import { invalidate } from "./lib";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { props: { onStart: () => invalidate(true) } }), "component", App, 6, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
