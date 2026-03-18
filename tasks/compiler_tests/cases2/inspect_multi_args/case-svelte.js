import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let a = 1;
	let b = 2;
	$.inspect(() => [a, b], (...$$args) => console.log(...$$args), true);
}
