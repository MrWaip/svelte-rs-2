import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	let refs = [];
	let items = [{ id: 1 }, { id: 2 }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		Comp($$renderer, {});
		$$renderer.push(`<!----> <button>x</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
