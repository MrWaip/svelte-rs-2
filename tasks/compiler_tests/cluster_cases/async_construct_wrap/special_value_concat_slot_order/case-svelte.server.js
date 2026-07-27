import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b } = $$props;
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(a))();
		$$renderer.option({ value: `x${$.stringify($$0)}` }, ($$renderer) => {
			$$renderer.push(async () => $.escape((await $.save(b))()));
		});
	});
}
