import * as $ from "svelte/internal/server";
export function f(a) {
	return a;
}
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}</button>`);
}
