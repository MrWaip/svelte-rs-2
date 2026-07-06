import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	let $$derived_array = $.derived(() => $.to_array(items)), first = $.derived(() => $$derived_array()[0]), second = $.derived(() => $$derived_array()[1]), rest = $.derived(() => $$derived_array().slice(2));
	$$renderer.push(`<p>${$.escape(first())},${$.escape(second())}</p>`);
}
