import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const noop = () => {};
	$$renderer.push(`<button>go</button>`);
}
