App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function tooltip(node) {
		return { destroy() {} };
	}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
		[$.attachment()]: tooltip,
		prop: "value"
	}), "component", App, 9, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
