import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	const B = $.derived(() => A);
	if (B()) {
		$$renderer.push("<!--[-->");
		B()($$renderer, {
			children: ($$renderer) => {
				$$renderer.push(`<!---->one`);
			},
			$$slots: { default: true }
		});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
	$$renderer.push(` `);
	if (B()) {
		$$renderer.push("<!--[-->");
		B()($$renderer, {
			children: ($$renderer) => {
				$$renderer.push(`<!---->two`);
			},
			$$slots: { default: true }
		});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
