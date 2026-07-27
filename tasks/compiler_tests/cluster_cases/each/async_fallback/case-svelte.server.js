import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		const each_array = $.ensure_array_like((await $.save(delay([n])))());
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
