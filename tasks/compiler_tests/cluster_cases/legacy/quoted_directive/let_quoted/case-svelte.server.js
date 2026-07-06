import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value;
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", {}, null);
	$$renderer.push(`<!--]-->`);
}
