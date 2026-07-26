import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 1;
		if (n) {
			$$renderer.push("<!--[0-->");
			let a;
			let b;
			let c;
			var promises = $$renderer.run([async () => a = (await $.save(Promise.resolve(n)))(), () => {
				b = $.derived(() => a * 2);
				c = a + 1;
			}]);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 8, 1);
			$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b())));
			$$renderer.push(` `);
			$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(c)));
			$$renderer.push(`</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
