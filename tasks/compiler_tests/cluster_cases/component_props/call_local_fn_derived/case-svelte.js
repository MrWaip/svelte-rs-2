import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	function getX() {
		return 1;
	}
	{
		let $0 = $.derived(getX);
		Child($$anchor, { get random() {
			return $.get($0);
		} });
	}
}
