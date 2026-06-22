import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	function fn(v) {
		return v + 1;
	}
	{
		let $0 = $.derived_safe_equal(() => ($.deep_read_state(value()), $.untrack(() => fn(value()))));
		Comp($$anchor, { get id() {
			return $.get($0);
		} });
	}
}
