import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> <!--[-->`);
	$$renderer.child_block(async ($$renderer) => {
		const $$0 = (await $.save(delay(x)))();
		$.slot($$renderer, $$props, "default", { value: $$0 }, null);
	});
	$$renderer.push(`<!--]-->`);
}
