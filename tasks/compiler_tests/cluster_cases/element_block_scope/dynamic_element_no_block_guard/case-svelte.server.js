import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>inc</button> <div>${$.escape(count)}</div>`);
}
