import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const p = { m: 1 };
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}${$.escape(p.m)}</button>`);
}
