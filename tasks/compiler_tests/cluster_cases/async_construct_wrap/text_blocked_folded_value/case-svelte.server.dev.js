import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 1;
		async function getDouble(value) {
			return value * 2;
		}
		var double;
		var $$promises = $$renderer.run([async () => double = await $.async_derived(() => getDouble(count))]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`Count: `);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(count)));
		$$renderer.push(` Double: `);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(double())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
