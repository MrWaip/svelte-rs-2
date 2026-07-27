import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function check(v) {
		return v > 0;
	}
	$$renderer.child_block(async ($$renderer) => {
		if ((await $.save(check(count)))()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>yes</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]--> <button>inc</button>`);
}
