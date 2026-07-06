import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [1], $$array = $.to_array(tmp, 2), a = $.fallback($$array[0], 10), b = $.fallback($$array[1], 20);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
