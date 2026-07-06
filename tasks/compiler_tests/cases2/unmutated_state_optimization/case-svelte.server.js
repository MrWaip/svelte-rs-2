import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let label = "hello";
	let items = [
		1,
		2,
		3
	];
	function increment() {
		count += 1;
	}
	$$renderer.push(`<button>${$.escape(count)}</button> <p>hello</p> <ul><!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<li>${$.escape(item)}</li>`);
	}
	$$renderer.push(`<!--]--></ul>`);
}
