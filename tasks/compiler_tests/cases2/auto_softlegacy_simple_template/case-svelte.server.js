import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	count += 1;
	$$renderer.push(`<p>${$.escape(count)}</p>`);
}
