import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function f() {
			return 1;
		}
		async function g() {
			return 2;
		}
		let { tag } = $$props;
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(f()))();
			Child($$renderer, {
				a: $$0,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$.validate_dynamic_element_tag(() => tag);
					$.push_element($$renderer, tag, 8, 1);
					$$renderer.child(async ($$renderer) => {
						const $$0 = (await $.save(g()))();
						$.element($$renderer, tag, () => {
							$$renderer.push(`${$.attr("title", $$0)}`);
						});
					});
					$.pop_element();
				}),
				$$slots: { default: true }
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
