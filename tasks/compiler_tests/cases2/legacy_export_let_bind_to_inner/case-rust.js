import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 12, "");
	Inner($$anchor, {
		get value() {
			return value;
		},
		set value($$value) {
			value = $$value;
		}
	});
}
