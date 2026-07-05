import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { items = [] } = $$props;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let [id, name] = each_array[$$index];
		$$renderer.push(`<p>${$.escape(name)}</p>`);
	}
	$$renderer.push(`<!--]--> <!--[-->`);
	const each_array_1 = $.ensure_array_like(items);
	for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
		let { id, name } = each_array_1[$$index_1];
		$$renderer.push(`<p>${$.escape(name)}</p>`);
	}
	$$renderer.push(`<!--]--> <!--[-->`);
	const each_array_2 = $.ensure_array_like(items);
	for (let idx = 0, $$length = each_array_2.length; idx < $$length; idx++) {
		let [id, name] = each_array_2[idx];
		$$renderer.push(`<p>${$.escape(idx)}: ${$.escape(name)}</p>`);
	}
	$$renderer.push(`<!--]--> <!--[-->`);
	const each_array_3 = $.ensure_array_like(items);
	for (let idx = 0, $$length = each_array_3.length; idx < $$length; idx++) {
		let [a, b, c] = each_array_3[idx];
	}
	$$renderer.push(`<!--]--> <!--[-->`);
	const each_array_4 = $.ensure_array_like(items);
	for (let idx = 0, $$length = each_array_4.length; idx < $$length; idx++) {
		let { a, b, c } = each_array_4[idx];
	}
	$$renderer.push(`<!--]-->`);
}
