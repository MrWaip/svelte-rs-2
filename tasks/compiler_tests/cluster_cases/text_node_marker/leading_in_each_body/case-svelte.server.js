import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let array = ["A"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(array);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let a = each_array[$$index];
		$$renderer.push(`<!---->${$.escape(a)}<br/>`);
	}
	$$renderer.push(`<!--]--> <button>add</button>`);
}
