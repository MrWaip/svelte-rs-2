import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function first() {
		return false;
	}
	async function second() {
		return true;
	}
	$$renderer.child_block(async ($$renderer) => {
		if ((await $.save(first()))()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>first</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.child_block(async ($$renderer) => {
				if ((await $.save(second()))()) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<p>second</p>`);
				} else {
					$$renderer.push("<!--[-1-->");
					$$renderer.push(`<p>fallback</p>`);
				}
			});
			$$renderer.push(`<!--]-->`);
		}
	});
	$$renderer.push(`<!--]-->`);
}
