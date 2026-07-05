App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag($.state(void 0), "v");
	function onx() {}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		a: "1",
		onx,
		b: "2",
		get value() {
			return $.get(v);
		},
		set value($$value) {
			$.set(v, $$value, true);
		}
	}), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
