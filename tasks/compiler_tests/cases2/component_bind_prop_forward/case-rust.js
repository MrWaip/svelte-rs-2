import * as $ from "svelte/internal/client";
import Input from "./Input.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 15);
	Input($$anchor, {
		get value() {
			return value();
		},
		set value($$value) {
			value($$value);
		}
	});
	$.pop();
}
