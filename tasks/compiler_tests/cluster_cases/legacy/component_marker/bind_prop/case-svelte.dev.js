import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.tag($.mutable_source(""), "value");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $.get(value);
		},
		set value($$value) {
			$.set(value, $$value);
		},
		$$legacy: true
	}), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
