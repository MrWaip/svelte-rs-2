import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function check() {
		return true;
	}
	$$renderer.child_block(async ($$renderer) => {
		if ((await $.save(check()))()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>yes</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]-->`);
}
