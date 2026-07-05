import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "hello";
	$$renderer.push(`<p>hello</p>`);
}
