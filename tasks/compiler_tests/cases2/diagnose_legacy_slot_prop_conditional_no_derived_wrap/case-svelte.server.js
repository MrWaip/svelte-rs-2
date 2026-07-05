import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let flag = $.fallback($$props["flag"], false);
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", { title: flag ? "A" : "B" }, null);
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { flag });
}
