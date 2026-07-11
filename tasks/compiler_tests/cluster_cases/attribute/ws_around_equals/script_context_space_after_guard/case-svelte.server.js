import * as $ from "svelte/internal/server";
export const svelte4space = "svelte4space";
export default function App($$renderer) {
	$$renderer.push(`<div>hi</div>`);
}
