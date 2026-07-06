App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		/** @type {{ name: string, count?: number }} */
		let { name, count = 0 } = $$props;
		/** @type {number} */
		let doubled = $.derived(() => count * 2);
		/** @type {number} */
		let label = $.derived(() => {
			// format with prefix
			return `${name}: ${doubled()}`;
		});
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 15, 0);
		$$renderer.push(`${$.escape(label())}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
