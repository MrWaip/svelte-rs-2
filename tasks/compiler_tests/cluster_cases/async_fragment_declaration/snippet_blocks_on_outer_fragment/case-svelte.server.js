import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	let number;
	function row($$renderer) {
		let doubled;
		var promises_1 = $$renderer.run([() => promises[0], () => doubled = number() * 2]);
		$$renderer.push(`<span>`);
		$$renderer.async([promises_1[1]], ($$renderer) => $$renderer.push(() => $.escape(doubled)));
		$$renderer.push(`</span>`);
	}
	var promises = $$renderer.run([async () => number = await $.async_derived(async () => (await $.save(Promise.resolve(n)))())]);
	row($$renderer);
}
