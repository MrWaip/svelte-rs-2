App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Container from "./Container.svelte";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = void 0 } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			Container($$renderer, {
				children: $.invalid_default_snippet,
				$$slots: { default: ($$renderer, { idx }) => {
					Child($$renderer, {
						get value() {
							return value[idx].name;
						},
						set value($$value) {
							value[idx].name = $$value;
							$$settled = false;
						}
					});
				} }
			});
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
