import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let handler = () => {};
	$$renderer.push(`<button>click</button>`);
}
