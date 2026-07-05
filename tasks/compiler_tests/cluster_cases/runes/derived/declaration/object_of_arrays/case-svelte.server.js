import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = {
		p: [1, 2],
		q: [3, 4]
	};
	let $$derived_array = $.derived(() => $.to_array(x.p, 2)), $$derived_array_1 = $.derived(() => $.to_array(x.q, 2)), a = $.derived(() => $$derived_array()[0]), b = $.derived(() => $$derived_array()[1]), c = $.derived(() => $$derived_array_1()[0]), d = $.derived(() => $$derived_array_1()[1]);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}${$.escape(c())}${$.escape(d())}</button>`);
}
