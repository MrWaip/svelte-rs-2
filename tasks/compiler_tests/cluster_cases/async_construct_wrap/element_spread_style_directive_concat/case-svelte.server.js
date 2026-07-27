import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function g() {
		return 1;
	}
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(g()))();
		$$renderer.push(`<div${$.attributes({ ...{ q: 1 } }, void 0, void 0, { color: `${$.stringify($$0)}px` })}></div>`);
	});
}
