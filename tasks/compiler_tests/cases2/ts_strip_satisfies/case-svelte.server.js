import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	count++;
	$$renderer.push(`<p>${$.escape(count)}</p>`);
}
