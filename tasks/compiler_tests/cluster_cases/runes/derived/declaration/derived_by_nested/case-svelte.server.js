import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = [[1, 2], [3, 4]];
	let $$d = $.derived(() => x), $$derived_array = $.derived(() => $.to_array($$d(), 2)), $$derived_array_1 = $.derived(() => $.to_array($$derived_array()[0], 2)), $$derived_array_2 = $.derived(() => $.to_array($$derived_array()[1], 2)), a = $.derived(() => $$derived_array_1()[0]), b = $.derived(() => $$derived_array_1()[1]), c = $.derived(() => $$derived_array_2()[0]), d = $.derived(() => $$derived_array_2()[1]);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}${$.escape(c())}${$.escape(d())}</button>`);
}
