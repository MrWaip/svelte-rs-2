import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = 0;
	$$renderer.push(`<button>x</button> ${$.escape(s)}`);
}
