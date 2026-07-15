import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><noscript><img src="x" alt=""/></noscript></div>`);
}
