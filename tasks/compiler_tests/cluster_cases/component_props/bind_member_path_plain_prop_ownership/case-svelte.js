import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let store = $.prop($$props, "store", 7);
	Comp($$anchor, {
		get value() {
			return store().inner.value;
		},
		set value($$value) {
			store().inner.value = $$value;
		}
	});
	$.pop();
}
