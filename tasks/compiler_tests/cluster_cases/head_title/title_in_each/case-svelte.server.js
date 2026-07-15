import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let i = each_array[$$index];
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>${$.escape(i)}</title>`);
			});
		}
		$$renderer.push(`<!--]-->`);
	});
	$.bind_props($$props, { items });
}
