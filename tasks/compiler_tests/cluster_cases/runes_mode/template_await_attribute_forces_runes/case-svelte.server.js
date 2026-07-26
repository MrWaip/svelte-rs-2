import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(compute(count)))();
		$$renderer.push(`<p${$.attr("title", $$0)}>hi</p>`);
	});
	$$renderer.push(` <button>inc</button>`);
}
