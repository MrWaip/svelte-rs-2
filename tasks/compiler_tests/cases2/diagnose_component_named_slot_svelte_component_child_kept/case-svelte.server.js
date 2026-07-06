import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
import A from "./A.svelte";
import B from "./B.svelte";
export default function App($$renderer, $$props) {
	let flag = $.fallback($$props["flag"], false);
	Inner($$renderer, { $$slots: { icon: ($$renderer) => {
		if (flag ? A : B) {
			$$renderer.push("<!--[-->");
			(flag ? A : B)($$renderer, { slot: "icon" });
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	} } });
	$.bind_props($$props, { flag });
}
