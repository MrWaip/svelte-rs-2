import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor) {
	let store = $.proxy({ inner: { value: 0 } });
	Comp($$anchor, {
		get value() {
			return store.inner.value;
		},
		set value($$value) {
			store.inner.value = $$value;
		}
	});
}
