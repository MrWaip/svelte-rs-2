import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const id = "name";
	{
		function greet($$renderer, x) {
			$$renderer.push(`<b>${$.escape(x)}</b>`);
		}
		let greeting2;
		var promises = $$renderer.run([async () => greeting2 = await $.async_derived(async () => (await $.save(`Hi ${id}`))())]);
		$$renderer.push(`<div><span>`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(greeting2())));
		$$renderer.push(`</span> `);
		greet($$renderer, 1);
		$$renderer.push(`<!----></div>`);
	}
}
