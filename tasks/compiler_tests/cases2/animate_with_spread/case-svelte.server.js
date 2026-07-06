import * as $ from "svelte/internal/server";
import { flip } from "svelte/animate";
export default function App($$renderer, $$props) {
	let { items = [], $$slots, $$events, ...rest } = $$props;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
		let item = each_array[idx];
		$$renderer.push(`<p${$.attributes({
			...rest,
			"data-index": `item-${$.stringify(idx)}`
		})}>${$.escape(item.name)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
