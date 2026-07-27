import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { tag } = $$props;
		async function g() {
			return 2;
		}
		$.validate_dynamic_element_tag(() => tag);
		$.push_element($$renderer, tag, 5, 0);
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save(g()))();
			$.element($$renderer, tag, () => {
				$$renderer.push(`${$.attr("title", $$0)}`);
			});
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
