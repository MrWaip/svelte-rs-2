import * as $ from "svelte/internal/server";
function row($$renderer) {
	$$renderer.push(`<p class="svelte-5iy3wu">hi</p>`);
}
export default function App($$renderer) {
	$$renderer.push(`<div class="wrap svelte-5iy3wu">`);
	row($$renderer);
	$$renderer.push(`<!----></div>`);
}
