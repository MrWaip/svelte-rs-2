import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let ref = $.tag($.mutable_source(), "ref");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => $.bind_this(Child($$anchor, { $$legacy: true }), ($$value) => $.set(ref, $$value), () => $.get(ref)), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
