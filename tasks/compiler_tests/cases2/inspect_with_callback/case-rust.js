import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let val = 0;
	$.inspect(() => [val], (...$$args) => console.warn(...$$args));
}
