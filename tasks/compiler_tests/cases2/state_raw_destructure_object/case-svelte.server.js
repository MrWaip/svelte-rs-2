import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		items: [],
		count: 0
	}, items = tmp.items, count = tmp.count;
	$$renderer.push(`<p>${$.escape(count)}</p>`);
}
