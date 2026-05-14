import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let v = $.state(void 0);
	function onx() {}
	Child($$anchor, {
		a: "1",
		onx,
		b: "2",
		get value() {
			return $.get(v);
		},
		set value($$value) {
			$.set(v, $$value, true);
		}
	});
}
