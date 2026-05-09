import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let inputRef = $.state(void 0);
	Child($$anchor, {
		get ref() {
			return $.get(inputRef);
		},
		set ref($$value) {
			$.set(inputRef, $$value, true);
		}
	});
}
