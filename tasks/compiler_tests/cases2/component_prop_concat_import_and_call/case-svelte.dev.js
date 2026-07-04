App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { BRAND } from "./brand";
import { getName } from "./helpers";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(() => BRAND);
		let $1 = $.derived(getName);
		$.add_svelte_meta(() => Comp($$anchor, { get title() {
			return `prefix ${$.get($0) ?? ""}${$.get($1) ?? ""}`;
		} }), "component", App, 7, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
