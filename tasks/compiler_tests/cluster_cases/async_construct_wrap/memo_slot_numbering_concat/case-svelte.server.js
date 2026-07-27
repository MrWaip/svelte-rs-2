import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function fn() {
		return 1;
	}
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save("x"))();
		$$renderer.push(`<div${$.attributes({
			...{},
			title: `a${$.stringify($$0)}b`,
			id: fn()
		})}>y</div>`);
	});
}
