import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let count = $.state(0);
	$.set(count, 1);
	{
		let $0 = $.derived(() => Math.max($.get(count), 2));
		Child($$anchor, { get random() {
			return $.get($0);
		} });
	}
}
