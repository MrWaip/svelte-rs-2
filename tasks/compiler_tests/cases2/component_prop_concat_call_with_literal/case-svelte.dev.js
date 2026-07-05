App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { getProductName } from "./helpers";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(getProductName);
		$.add_svelte_meta(() => Comp($$anchor, { get title() {
			return `x0${$.get($0) ?? ""}`;
		} }), "component", App, 6, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
