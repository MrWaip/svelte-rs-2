import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const id = "name";
	{
		const nested = "nested";
		let greeting2;
		var promises = $$renderer.run([async () => greeting2 = await $.async_derived(async () => (await $.save(`Hi ${id}`))())]);
		$$renderer.push(`<div><span>nested `);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(greeting2())));
		$$renderer.push(`</span></div>`);
	}
}
