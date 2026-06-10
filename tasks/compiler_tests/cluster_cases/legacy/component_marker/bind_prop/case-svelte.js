import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let value = $.mutable_source("");
	Child($$anchor, {
		get value() {
			return $.get(value);
		},
		set value($$value) {
			$.set(value, $$value);
		},
		$$legacy: true
	});
}
