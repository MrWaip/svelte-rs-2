App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const SIZE = 100;
	const RADIUS = SIZE / 2;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		a: SIZE,
		b: RADIUS
	}), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
