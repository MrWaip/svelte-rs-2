import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="hit svelte-17fn2ym">inside</div> <div>outside</div>`);
}
