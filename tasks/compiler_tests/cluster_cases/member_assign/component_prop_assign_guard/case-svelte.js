import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	{
		let $0 = $.derived(() => obj.x = src);
		Child($$anchor, { get prop() {
			return $.get($0);
		} });
	}
}
