import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let toggle = $.prop($$props, "toggle", 8, false);
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => toggle() ? "A" : "B");
		$.add_svelte_meta(() => Comp($$anchor, { get description() {
			return `prefix ${$.get($0) ?? ""}`;
		} }), "component", App, 6, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
