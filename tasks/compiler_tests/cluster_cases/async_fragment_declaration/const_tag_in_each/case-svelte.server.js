import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [1, 2];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		let loaded;
		var promises = $$renderer.run([async () => loaded = (await $.save(Promise.resolve(item)))()]);
		$$renderer.push(`<p>`);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded)));
		$$renderer.push(`</p>`);
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
