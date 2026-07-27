import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(Promise.resolve("x")))();
		$$renderer.select({ value: $$0 }, ($$renderer) => {
			$$renderer.child(async ($$renderer) => {
				const $$0 = (await $.save(Promise.resolve("y")))();
				$$renderer.option({ value: $$0 }, ($$renderer) => {
					$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a)));
				});
			});
		});
	});
}
