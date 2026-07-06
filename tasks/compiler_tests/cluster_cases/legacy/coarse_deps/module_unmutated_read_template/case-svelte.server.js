import * as $ from "svelte/internal/server";
let label = "hi";
export default function App($$renderer) {
	$$renderer.push(`<p>hi</p>`);
}
