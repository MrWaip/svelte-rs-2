import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<span data-x="1"></span>`);
}
