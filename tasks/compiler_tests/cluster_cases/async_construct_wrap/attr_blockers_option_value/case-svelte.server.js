import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
	$$renderer.push(`<select>`);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.option({ value: a }, ($$renderer) => {
			$$renderer.push(`x`);
		});
	});
	$$renderer.push(`</select>`);
}
