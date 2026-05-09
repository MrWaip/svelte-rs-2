import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let base = false;
	let derivedFlag = $.derived(() => base);
	Child($$anchor, {
		get value() {
			return $.get(derivedFlag);
		},
		set value($$value) {
			$.set(derivedFlag, $$value);
		}
	});
}
