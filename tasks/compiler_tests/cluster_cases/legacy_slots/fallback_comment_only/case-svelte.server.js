import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "a", {}, () => {});
	$$renderer.push(`<!--]-->`);
}
