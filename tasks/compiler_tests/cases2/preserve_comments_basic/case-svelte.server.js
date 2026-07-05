import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><!-- hello world --> <span>after</span></div>`);
}
