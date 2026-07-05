import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	$$renderer.push(`<p>${$.escape(a)}<br/></p> <button>inc</button>`);
}
