import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded;
	var $$promises = $$renderer.run([async () => loaded = await delay([1, 2])]);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		const each_array = $.ensure_array_like(loaded);
		if (each_array.length !== 0) {
			$$renderer.push("<!--[-->");
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				$$renderer.push(`<p>${$.escape(item)}</p>`);
			}
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push(`<p>empty</p>`);
		}
	});
	$$renderer.push(`<!--]-->`);
}
