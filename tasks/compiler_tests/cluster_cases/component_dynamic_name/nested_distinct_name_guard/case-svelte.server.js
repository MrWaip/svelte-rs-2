import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	const B = $.derived(() => A);
	const C = $.derived(() => A);
	if (B()) {
		$$renderer.push("<!--[-->");
		B()($$renderer, {
			children: ($$renderer) => {
				if (C()) {
					$$renderer.push("<!--[-->");
					C()($$renderer, {
						children: ($$renderer) => {
							$$renderer.push(`<!---->test`);
						},
						$$slots: { default: true }
					});
					$$renderer.push("<!--]-->");
				} else {
					$$renderer.push("<!--[!-->");
					$$renderer.push("<!--]-->");
				}
			},
			$$slots: { default: true }
		});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
