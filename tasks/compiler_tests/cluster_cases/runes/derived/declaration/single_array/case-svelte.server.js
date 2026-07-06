import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = [1, 2];
	let $$derived_array = $.derived(() => $.to_array(x, 2)), a = $.derived(() => $$derived_array()[0]), b = $.derived(() => $$derived_array()[1]);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}</button>`);
}
