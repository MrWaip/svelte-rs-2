import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = [[1, 2], 3];
	let $$derived_array = $.derived(() => $.to_array(x, 2)), $$derived_array_1 = $.derived(() => $.to_array($.fallback($$derived_array()[0], () => [9, 9], true), 2)), a = $.derived(() => $$derived_array_1()[0]), b = $.derived(() => $$derived_array_1()[1]), c = $.derived(() => $$derived_array()[1]);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}${$.escape(c())}</button>`);
}
