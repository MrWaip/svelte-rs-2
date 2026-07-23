App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Comp($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				const foo = "bar";
				{
					$.prevent_snippet_stringification(prop);
					function prop($$renderer) {
						$.validate_snippet_args($$renderer);
						$$renderer.push(`<!---->bar`);
					}
					Inner($$renderer, {
						prop,
						$$slots: { prop: true }
					});
				}
			}),
			$$slots: { default: true }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
