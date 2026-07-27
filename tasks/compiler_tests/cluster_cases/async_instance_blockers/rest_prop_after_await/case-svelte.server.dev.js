import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function inc() {
			c++;
		}
		var x, a, b, c, rest;
		var $$promises = $$renderer.run([async () => x = await Promise.resolve(1), () => ({a, b = 2, c = 3, $$slots, $$events, ...rest} = $$props)]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(x)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(a)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(c)));
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(JSON.stringify(rest))));
		$$renderer.push(`</button>`);
		$.pop_element();
		$.bind_props($$props, { c });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
