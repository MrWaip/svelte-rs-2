import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let foo = $.prop($$props, "foo", 15);
	Child($$anchor, {
		get value() {
			return foo();
		},
		set value($$value) {
			foo($$value);
		}
	});
	$.pop();
}
