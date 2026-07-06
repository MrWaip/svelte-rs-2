import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="x"><input value="x"/></div>`);
}
