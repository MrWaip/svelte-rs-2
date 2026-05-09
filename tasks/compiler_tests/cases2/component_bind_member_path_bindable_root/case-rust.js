import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let store = $.prop($$props, "store", 15);
	Comp($$anchor, {
		get value() {
			return store().inner.value;
		},
		set value($$value) {
			store(store().inner.value = $$value, true);
		}
	});
	$.pop();
}
