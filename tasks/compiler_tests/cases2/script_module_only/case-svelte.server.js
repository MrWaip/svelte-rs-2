import * as $ from "svelte/internal/server";
export const VERSION = "1.0.0";
export default function App($$renderer) {
	$$renderer.push(`<p>Static content</p>`);
}
