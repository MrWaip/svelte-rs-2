import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<svg><circle cx="50" cy="50" r="50"></circle></svg>`);
}
