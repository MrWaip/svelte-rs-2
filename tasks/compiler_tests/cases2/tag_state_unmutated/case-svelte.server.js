import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { a: 1 };
	$$renderer.push(`<p>${$.escape(obj.a)}</p>`);
}
