import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
function pending($$renderer) {
	$$renderer.push(`<!---->loading`);
}
export default function App($$renderer) {
	if (pending) {
		$$renderer.push(`<!--[!-->`);
		pending($$renderer);
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push(`<!--[-->`);
		{
			let data;
			var promises = $$renderer.run([async () => data = (await $.save(Promise.resolve("d")))()]);
			$$renderer.push(`<!---->`);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data)));
		}
		$$renderer.push(`<!--]-->`);
	}
}
