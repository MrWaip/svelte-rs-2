import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.select({ value: a }, ($$renderer) => {
			$$renderer.option({}, ($$renderer) => {
				$$renderer.push(`x`);
			});
		});
	});
}
