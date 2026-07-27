import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(Promise.resolve("dog")))();
		$$renderer.select({ value: $$0 }, ($$renderer) => {
			$$renderer.option({}, ($$renderer) => {
				$$renderer.push(`--Please choose an option--`);
			});
			$$renderer.child(async ($$renderer) => {
				const $$0 = (await $.save(Promise.resolve("dog")))();
				$$renderer.option({}, $$0);
			});
			$$renderer.option({}, ($$renderer) => {
				$$renderer.push(`cat`);
			});
		});
	});
}
