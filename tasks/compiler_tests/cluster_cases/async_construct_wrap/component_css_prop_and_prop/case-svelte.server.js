import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		const [$$0, $$1] = (await $.save(Promise.all([(async () => (await $.save(delay(x)))())(), (async () => (await $.save(delay(x + 1)))())()])))();
		$.css_props($$renderer, true, { "--color": $$0 }, () => {
			Child($$renderer, { value: $$1 });
		});
	});
}
