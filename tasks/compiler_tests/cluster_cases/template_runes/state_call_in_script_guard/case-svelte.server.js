import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = { a: 1 };
	$$renderer.push(`<button>${$.escape(x.a)}</button>`);
}
