import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="a" id="b"></div>`);
}
