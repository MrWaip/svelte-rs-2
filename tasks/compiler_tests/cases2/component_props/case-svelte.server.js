import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
export default function App($$renderer) {
	let count = 0;
	function handler() {
		count++;
	}
	Button($$renderer, {
		label: "Click me",
		onclick: handler,
		count
	});
}
