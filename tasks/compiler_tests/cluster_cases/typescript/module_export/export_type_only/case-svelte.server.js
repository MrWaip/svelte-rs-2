import * as $ from "svelte/internal/server";
export const K = 1;
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}1</button>`);
}
