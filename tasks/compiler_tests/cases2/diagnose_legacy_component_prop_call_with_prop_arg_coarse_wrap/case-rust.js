import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Badge from "./Badge.svelte";
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	function f(n) {
		return n;
	}
	{
		let $0 = $.derived_safe_equal(() => ($.deep_read_state(x()), $.untrack(() => f(x()))));
		Badge($$anchor, { get text() {
			return `a ${$.get($0) ?? ""} b`;
		} });
	}
}
