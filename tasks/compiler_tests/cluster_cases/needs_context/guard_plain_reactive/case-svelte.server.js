import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 0;
	$$renderer.push(`<button>${$.escape(n)}</button>`);
}
