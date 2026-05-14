import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	function build(v) {
		return { value: v };
	}
	{
		let $0 = $.derived_safe_equal(() => build(value()));
		Child($$anchor, $.spread_props(() => $.get($0)));
	}
}
