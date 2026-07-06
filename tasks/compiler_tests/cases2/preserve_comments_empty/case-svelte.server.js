import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><!----> <span>after</span></div>`);
}
