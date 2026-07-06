import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let numbers = [
		1,
		2,
		3
	];
	function x($$renderer, n) {
		$$renderer.push(`<p>${$.escape(n)}</p>`);
	}
	$$renderer.push(`<div><!--[-->`);
	const each_array = $.ensure_array_like(numbers);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let n = each_array[$$index];
		x($$renderer, n);
	}
	$$renderer.push(`<!--]--></div>`);
}
