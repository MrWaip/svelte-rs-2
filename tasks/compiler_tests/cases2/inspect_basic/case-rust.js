import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = 0;
	$.inspect(() => [count], (...$$args) => console.log(...$$args), true);
}
