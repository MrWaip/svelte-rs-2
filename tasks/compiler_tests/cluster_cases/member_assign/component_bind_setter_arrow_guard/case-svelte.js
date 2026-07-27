import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	let v = 0;
	var bind_get = () => v;
	var bind_set = (n) => obj.x = src;
	Child($$anchor, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		}
	});
}
