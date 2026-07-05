App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const obj = $.tag($.derived(() => ({ "data-testid": "x" })), "obj");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Comp($$anchor, { get dataTestid() {
		return $.get(obj)["data-testid"];
	} }), "component", App, 4, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
