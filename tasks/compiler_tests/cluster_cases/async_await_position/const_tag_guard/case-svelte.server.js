import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve({ value });
		}
		$$renderer.push(`<button>inc</button> <!--[-->`);
		const each_array = $.ensure_array_like([1]);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			let current;
			var promises = $$renderer.run([async () => current = (await $.save(delay(x)))().value]);
			$$renderer.push(`<p>`);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(current)));
			$$renderer.push(`${$.escape(item)}</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
