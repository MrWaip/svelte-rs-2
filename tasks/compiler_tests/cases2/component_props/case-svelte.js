import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor) {
	let count = $.state(0);
	function handler() {
		$.update(count);
	}
	Button($$anchor, {
		label: "Click me",
		onclick: handler,
		get count() {
			return $.get(count);
		}
	});
}
