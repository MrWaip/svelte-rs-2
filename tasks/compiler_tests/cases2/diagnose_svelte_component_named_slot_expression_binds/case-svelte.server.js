import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	let current = Inner;
	if (current) {
		$$renderer.push("<!--[-->");
		current($$renderer, { $$slots: { caption: ($$renderer) => {
			$$renderer.push(`<span slot="caption">hi</span>`);
		} } });
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
