import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	let arr = [1, 2];
	var squared, cubed, toFixed, toString, a, b;
	var $$promises = $$renderer.run([async () => {
		var $$d = await $.async_derived(() => ({
			squared: count ** 2,
			cubed: count ** 3
		}));
		squared = $.derived(() => $$d().squared);
		cubed = $.derived(() => $$d().cubed);
	}, () => {
		toFixed = $.derived(() => count.toFixed);
		toString = $.derived(() => count.toString);
		var $$derived_array = $.derived(() => $.to_array(arr, 2));
		a = $.derived(() => $$derived_array()[0]);
		b = $.derived(() => $$derived_array()[1]);
	}]);
	$$renderer.push(`<button>increment</button> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(count)));
	$$renderer.push(` ** 2 = `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(squared())));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(count)));
	$$renderer.push(` ** 3 = `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(cubed())));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(typeof toFixed())));
	$$renderer.push(` `);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(typeof toString())));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a())));
	$$renderer.push(` `);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b())));
	$$renderer.push(`</p>`);
}
