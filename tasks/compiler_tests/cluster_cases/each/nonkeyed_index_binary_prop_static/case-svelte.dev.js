import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let list = $.prop($$props, "list", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => list() || [], $.index, ($$anchor, item, idx) => {
		$.add_svelte_meta(() => Child($$anchor, { label: `ID (${idx + 1})` }), "component", App, 9, 1, { componentTag: "Child" });
	}), "each", App, 8, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
