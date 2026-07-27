import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a;
	var $$promises = $$renderer.run([async () => a = await Promise.resolve([1])]);
	$$renderer.push(`<!--[-->`);
	{
		let b;
		var promises = $$renderer.run([async () => b = (await $.save(Promise.resolve([2])))()]);
		$$renderer.push(`<!--[-->`);
		$$renderer.async_block([promises[0], $$promises[0]], ($$renderer) => {
			const each_array = $.ensure_array_like([...b, ...a]);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let x = each_array[$$index];
				$$renderer.push(`<p>${$.escape(x)}</p>`);
			}
		});
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
}
