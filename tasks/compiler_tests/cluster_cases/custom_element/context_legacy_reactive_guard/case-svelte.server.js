import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let y;
	let x = 1;
	y;
	$: y = x * 2;
}
