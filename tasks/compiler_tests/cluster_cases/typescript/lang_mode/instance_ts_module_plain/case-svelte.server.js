import * as $ from "svelte/internal/server";
export const K = { m: 1 };
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}${$.escape(K.m)}</button>`);
}
