import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	let Holder;
	let flag = $$props["flag"];
	$: Holder = { component: Inner };
	if (flag) {
		$$renderer.push("<!--[0-->");
		Holder.component($$renderer, {});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { flag });
}
