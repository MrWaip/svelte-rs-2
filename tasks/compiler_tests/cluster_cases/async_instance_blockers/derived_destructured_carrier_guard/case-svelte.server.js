import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let arr = [
		1,
		2,
		3
	];
	var a, rest, x, others;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => {
		var $$derived_array = $.derived(() => $.to_array(arr));
		a = $.derived(() => $$derived_array()[0]);
		rest = $.derived(() => $$derived_array().slice(1));
		var $$d = $.derived(() => ({ x: 2 }));
		x = $.derived(() => $.fallback($$d().x, 1));
		others = $.derived(() => $.exclude_from_object($$d(), ["x"]));
	}]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a())));
	$$renderer.push(` `);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(rest().length)));
	$$renderer.push(` `);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(x())));
	$$renderer.push(` `);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(others().y)));
	$$renderer.push(`</p>`);
}
