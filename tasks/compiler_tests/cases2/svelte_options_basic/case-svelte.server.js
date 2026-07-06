import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<p>Hello world</p>`);
}
