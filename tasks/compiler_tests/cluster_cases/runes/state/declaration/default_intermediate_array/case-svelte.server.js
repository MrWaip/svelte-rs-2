import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [[1, 2], 3], $$array = $.to_array(tmp, 2), $$array_1 = $.to_array($.fallback($$array[0], () => [8, 9], true), 2), a = $$array_1[0], b = $$array_1[1], c = $$array[1];
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
}
