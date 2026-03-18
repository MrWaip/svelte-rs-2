import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	{
		let $0 = $.derived(getData);
		Comp($$anchor, {
			label: "hello",
			get data() {
				return $.get($0);
			},
			get count() {
				return $$props.count;
			}
		});
	}
}
