import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
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
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 17, 0);
		$$renderer.push(`increment</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 19, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(count)));
		$$renderer.push(` ** 2 = `);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(squared())));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 20, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(count)));
		$$renderer.push(` ** 3 = `);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(cubed())));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 21, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(typeof toFixed())));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(typeof toString())));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 22, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a())));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
