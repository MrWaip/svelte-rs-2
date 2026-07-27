App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Comp($$anchor, { prop: "a" }), "component", App, 1, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
