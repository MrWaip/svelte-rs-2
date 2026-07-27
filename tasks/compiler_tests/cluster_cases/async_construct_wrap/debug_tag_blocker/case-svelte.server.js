import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	{
		let data;
		var promises = $$renderer.run([async () => data = (await $.save(Promise.resolve("works")))()]);
		$$renderer.async_block([promises[0]], ($$renderer) => {
			console.log({ data });
			debugger;
		});
	}
	$$renderer.push(`<!--]-->`);
}
