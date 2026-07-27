import * as $ from "svelte/internal/server";
import A from "./A.svelte";
import B from "./B.svelte";
export default function App($$renderer) {
	let value = 0;
	let Comp = $.derived(() => value % 2 === 0 ? A : B);
	$.css_props($$renderer, true, { "--prop": "red" }, () => {
		if (Comp()) {
			$$renderer.push("<!--[-->");
			Comp()($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, true);
}
