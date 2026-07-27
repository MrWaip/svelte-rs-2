import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	async function f() {
		return 1;
	}
	$$renderer.child_block(async ($$renderer) => {
		const $$0 = (await $.save(f()))();
		$.css_props($$renderer, true, { "--c": "1px" }, () => {
			Child($$renderer, { a: `y${$.stringify($$0)}` });
		});
	});
}
