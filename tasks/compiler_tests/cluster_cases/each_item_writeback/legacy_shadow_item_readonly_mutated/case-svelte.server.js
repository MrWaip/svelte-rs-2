import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = ["Hello"];
	function go() {
		a = [...a, "x"];
	}
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(a);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let a = each_array[$$index];
		$$renderer.push(`<!---->${$.escape(a)}`);
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
