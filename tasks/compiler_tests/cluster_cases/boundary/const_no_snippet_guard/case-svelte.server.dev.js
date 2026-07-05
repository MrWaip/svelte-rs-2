App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { n } = $$props;
		function compute() {
			return n + 1;
		}
		$$renderer.push(`<!--[-->`);
		{
			const value = compute();
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 10, 1);
			$$renderer.push(`${$.escape(value)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
