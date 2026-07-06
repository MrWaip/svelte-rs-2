import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<svg><circle r="5"></circle></svg>`);
}
