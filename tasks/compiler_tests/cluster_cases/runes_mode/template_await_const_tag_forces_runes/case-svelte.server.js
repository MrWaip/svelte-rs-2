import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	const promise = Promise.resolve(1);
	if (count >= 0) {
		$$renderer.push("<!--[0-->");
		let value;
		var promises = $$renderer.run([async () => value = (await $.save(promise))()]);
		$$renderer.push(`<p>`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(value)));
		$$renderer.push(` ${$.escape(count)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>inc</button>`);
}
