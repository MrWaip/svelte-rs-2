import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const binding_group = [];
	let value = $.state("a");
	Child($$anchor, {
		get group() {
			return $.get(value);
		},
		set group($$value) {
			$.set(value, $$value, true);
		}
	});
}
