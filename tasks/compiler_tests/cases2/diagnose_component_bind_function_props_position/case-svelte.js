import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor) {
	let value = $.state(0);
	var bind_get = () => $.get(value);
	var bind_set = (v) => $.set(value, v, true);
	Comp($$anchor, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		},
		label: "x",
		id: "y"
	});
}
