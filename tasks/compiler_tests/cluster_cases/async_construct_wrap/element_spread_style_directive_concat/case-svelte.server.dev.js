import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function g() {
			return 1;
		}
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save(g()))();
			$$renderer.push(`<div${$.attributes({ ...{ q: 1 } }, void 0, void 0, { color: `${$.stringify($$0)}px` })}>`);
			$.push_element($$renderer, "div", 4, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
