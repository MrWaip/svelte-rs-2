import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><!-- a --> <!-- b --> <span>tail</span></div>`);
}
