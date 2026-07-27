import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.select({ value: 42 }, ($$renderer) => {
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save(Promise.resolve(42)))();
			$$renderer.option({}, $$0);
		});
	});
}
