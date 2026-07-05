import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	let current = A;
	let cond = false;
	if (current) {
		$$renderer.push("<!--[-->");
		current($$renderer, {
			children: ($$renderer) => {
				if (cond) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<span>child</span>`);
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
