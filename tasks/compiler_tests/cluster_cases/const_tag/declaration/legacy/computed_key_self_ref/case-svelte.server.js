import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let boxes = [{ k1: "a" }];
	let area = "";
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(boxes);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let box = each_array[$$index];
		const { i = 1, [`k${i}`]: sideone, [`k${area}${i + 1}`]: sidetwo } = box;
		$$renderer.push(`<button>${$.escape(sideone)}${$.escape(sidetwo)}${$.escape(i)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
