import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let item = "hello";
	function get_item() {
		return item;
	}
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", { item: get_item() }, null);
	$$renderer.push(`<!--]-->`);
}
