import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		function pick(first, second) {
			return first + second;
		}
		function wrap(object) {
			return object.value;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 17, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.push(async () => $.escape(pick(1, await delay(x))));
		$$renderer.push(`
`);
		$$renderer.push(async () => $.escape(x > 0 ? await delay(x) : 0));
		$$renderer.push(`
`);
		$$renderer.push(async () => $.escape(`value ${await delay(x)}`));
		$$renderer.push(`
`);
		$$renderer.push(async () => $.escape((0, await delay(x))));
		$$renderer.push(`
`);
		$$renderer.push(async () => $.escape(wrap({
			index: 1,
			value: await delay(x)
		})));
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
