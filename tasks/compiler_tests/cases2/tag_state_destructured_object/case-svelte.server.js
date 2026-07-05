import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = { pair: [1, 2] }, $$array = $.to_array(tmp.pair, 2), a = $$array[0], b = $$array[1];
	a = 10;
	b = 20;
	$$renderer.push(`<p>${$.escape(a)} ${$.escape(b)}</p>`);
}
