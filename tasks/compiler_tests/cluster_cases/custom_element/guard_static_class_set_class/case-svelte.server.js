import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<my-el class="foo"></my-el>`);
}
