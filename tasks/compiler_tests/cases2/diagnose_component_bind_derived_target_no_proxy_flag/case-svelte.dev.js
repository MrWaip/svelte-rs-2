App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let base = false;
	let derivedFlag = $.tag($.derived(() => base), "derivedFlag");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $.get(derivedFlag);
		},
		set value($$value) {
			$.set(derivedFlag, $$value);
		}
	}), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
