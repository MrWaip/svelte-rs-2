import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = [
		1,
		2,
		3
	];
	let $$derived_array = $.derived(() => $.to_array(x)), $$derived_array_1 = $.derived(() => $.to_array($$derived_array().slice(1), 2)), a = $.derived(() => $$derived_array()[0]), b = $.derived(() => $$derived_array_1()[0]), c = $.derived(() => $$derived_array_1()[1]);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}${$.escape(c())}</button>`);
}
