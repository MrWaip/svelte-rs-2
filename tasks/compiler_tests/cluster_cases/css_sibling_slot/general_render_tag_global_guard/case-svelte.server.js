import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><p class="before svelte-105a842">before</p> `);
	children($$renderer);
	$$renderer.push(`<!----> <p class="foo svelte-105a842"><span class="svelte-105a842">foo</span></p> <p class="bar svelte-105a842">bar</p></div>`);
}
