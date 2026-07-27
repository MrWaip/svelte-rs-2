import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a } = $$props;
		let g = void 0;
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save(a))();
			$$renderer.push(`<input type="checkbox"${$.attr("checked", g, true)}${$.attr("value", $$0)}/>`);
			$.push_element($$renderer, "input", 2, 0);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
