import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	let go = false;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	if (go) {
		$$renderer.push("<!--[0-->");
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(delay(x)))();
			App($$renderer, { value: $$0 });
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
