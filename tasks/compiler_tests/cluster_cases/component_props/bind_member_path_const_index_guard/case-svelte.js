import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 15);
	const idx = 0;
	Child($$anchor, {
		get value() {
			return value()[idx].name;
		},
		set value($$value) {
			value(value()[idx].name = $$value, true);
		}
	});
	$.pop();
}
