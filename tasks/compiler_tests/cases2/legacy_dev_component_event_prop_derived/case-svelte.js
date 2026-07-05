import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	function handleClick() {}
	function getHandler() {
		return handleClick;
	}
	{
		let $0 = $.derived_safe_equal(() => $.untrack(getHandler));
		Child($$anchor, { get onclick() {
			return $.get($0);
		} });
	}
}
