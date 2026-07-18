import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value } = $$props;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like([1, 2]);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let n = each_array[$$index];
		const value = n * 10;
		$$renderer.push(`<p>${$.escape(value)}</p>`);
	}
	$$renderer.push(`<!--]--> <span>${$.escape(value)}</span>`);
}
