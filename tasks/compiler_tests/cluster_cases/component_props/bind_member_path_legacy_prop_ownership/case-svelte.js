import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let step = $.prop($$props, "step", 12);
	$.init();
	Child($$anchor, {
		get value() {
			return step().editValue;
		},
		set value($$value) {
			step(step().editValue = $$value, true);
		},
		$$legacy: true
	});
	$.pop();
}
