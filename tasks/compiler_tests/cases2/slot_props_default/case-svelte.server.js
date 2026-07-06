import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let entry = "hello";
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", { item: entry }, null);
	$$renderer.push(`<!--]-->`);
}
