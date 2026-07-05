import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 7);
	Child($$anchor, {
		get value() {
			return value();
		},
		set value($$value) {
			value($$value);
		}
	});
}
