import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { getProductName } from "./helpers";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.init();
	{
		let $0 = $.derived_safe_equal(getProductName);
		$.add_svelte_meta(() => Comp($$anchor, { get title() {
			return `${$.get($0) ?? ""} suffix`;
		} }), "component", App, 6, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
