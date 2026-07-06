import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	const cleanup = () => {};
	$$renderer.push(`<p>0</p>`);
}
