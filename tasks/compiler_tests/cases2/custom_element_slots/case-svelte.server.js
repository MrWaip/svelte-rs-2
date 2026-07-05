import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<header><!--[-->`);
	$.slot($$renderer, $$props, "actions", {}, null);
	$$renderer.push(`<!--]--></header> <main><!--[-->`);
	$.slot($$renderer, $$props, "default", {}, null);
	$$renderer.push(`<!--]--></main>`);
}
