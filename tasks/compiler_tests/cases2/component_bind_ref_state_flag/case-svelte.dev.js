App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let inputRef = $.tag($.state(void 0), "inputRef");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get ref() {
			return $.get(inputRef);
		},
		set ref($$value) {
			$.set(inputRef, $$value, true);
		}
	}), "component", App, 6, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
