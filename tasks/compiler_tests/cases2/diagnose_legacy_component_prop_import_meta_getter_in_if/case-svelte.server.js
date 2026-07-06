import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let cond = $$props["cond"];
		if (cond) {
			$$renderer.push("<!--[0-->");
			Inner($$renderer, { url: import.meta.env.VITE_X });
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { cond });
	});
}
