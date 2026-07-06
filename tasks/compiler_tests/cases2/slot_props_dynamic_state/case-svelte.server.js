import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let item = $.fallback($$props["item"], "hello");
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", { item }, null);
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { item });
}
