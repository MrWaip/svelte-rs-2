import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor) {
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	{
		let $0 = $.derived(getData);
		Comp($$anchor, { get data() {
			return $.get($0);
		} });
	}
}
