import * as $ from "svelte/internal/server";
import Component from "./Component.svelte";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	let refs = [];
	let obj = { ref: null };
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		Component($$renderer, {});
	}
	$$renderer.push(`<!--]--> `);
	Component($$renderer, {});
	$$renderer.push(`<!---->`);
}
