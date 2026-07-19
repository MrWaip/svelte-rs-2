import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<div><p class="before svelte-105a842">before</p> <!--[-->`);
	$.slot($$renderer, $$props, "default", {}, null);
	$$renderer.push(`<!--]--> <p class="foo svelte-105a842"><span class="svelte-105a842">foo</span></p> <p class="bar svelte-105a842">bar</p></div>`);
}
