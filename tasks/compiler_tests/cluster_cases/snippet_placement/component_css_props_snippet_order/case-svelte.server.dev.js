App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		$.css_props($$renderer, true, { "--my-var": "baseline" }, () => {
			{
				$.prevent_snippet_stringification(element);
				function element($$renderer, { idx }) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<div>`);
					$.push_element($$renderer, "div", 8, 2);
					$$renderer.push(`${$.escape(idx)}</div>`);
					$.pop_element();
				}
				Child($$renderer, {
					value: data,
					element,
					$$slots: { element: true }
				});
			}
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
