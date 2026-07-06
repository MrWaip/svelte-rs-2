import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<p>Static content</p>`);
}
