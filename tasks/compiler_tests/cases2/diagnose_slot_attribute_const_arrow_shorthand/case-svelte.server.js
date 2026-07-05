import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const onClose = () => {};
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "header", { onClose }, null);
	$$renderer.push(`<!--]-->`);
}
