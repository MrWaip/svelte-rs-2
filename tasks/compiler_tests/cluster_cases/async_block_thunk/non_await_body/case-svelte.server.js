import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	function delay_list(value) {
		return Promise.resolve([value]);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		if ((await $.save(delay(x)))() > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>positive</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]--> <!--[-->`);
	$$renderer.child_block(async ($$renderer) => {
		const each_array = $.ensure_array_like((await $.save(delay_list(x)))() || []);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<p>${$.escape(item)}</p>`);
		}
	});
	$$renderer.push(`<!--]--> <!--[--><!---->`);
	{
		$$renderer.push(`<p>keyed</p>`);
	}
	$$renderer.push(`<!----><!--]--> `);
	$$renderer.child_block(async ($$renderer) => {
		$.await($$renderer, (async () => (await $.save(delay(x)))() + 1)(), () => {}, (value) => {
			$$renderer.push(`<p>${$.escape(value)}</p>`);
		});
	});
	$$renderer.push(`<!--]-->`);
}
