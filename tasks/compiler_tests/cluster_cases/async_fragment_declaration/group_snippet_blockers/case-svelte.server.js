import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (true) {
		$$renderer.push("<!--[0-->");
		let number;
		function greet($$renderer) {
			$$renderer.push(`<h1>`);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(number)));
			$$renderer.push(`</h1>`);
		}
		var promises = $$renderer.run([async () => number = (await $.save(Promise.resolve(5)))()]);
		greet($$renderer);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
