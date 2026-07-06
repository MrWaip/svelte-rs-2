import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let elem = void 0;
		if (Inner) {
			$$renderer.push("<!--[-->");
			Inner($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	});
}
