import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 12);
	var bind_get = () => value();
	var bind_set = (v) => value(v?.trim());
	Comp($$anchor, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		},
		$$legacy: true
	});
}
