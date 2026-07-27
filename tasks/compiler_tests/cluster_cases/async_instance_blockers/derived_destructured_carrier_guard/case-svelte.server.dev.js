import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
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
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a())));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(rest().length)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(x())));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(others().y)));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
