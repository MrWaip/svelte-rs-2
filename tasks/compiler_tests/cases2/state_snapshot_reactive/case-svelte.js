import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = 0;
	let snap = $.snapshot(count);
}
