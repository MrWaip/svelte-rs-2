import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	let count = 0;
	function getHandler() {
		return () => count++;
	}
	Widget($$renderer, {});
}
