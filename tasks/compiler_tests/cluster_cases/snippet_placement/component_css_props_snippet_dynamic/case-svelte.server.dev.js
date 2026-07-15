App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { Comp } = $$props;
		$.css_props($$renderer, true, { "--my-var": "baseline" }, () => {
			{
				$.prevent_snippet_stringification(element);
				function element($$renderer, { idx }) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<div>`);
					$.push_element($$renderer, "div", 7, 2);
					$$renderer.push(`${$.escape(idx)}</div>`);
					$.pop_element();
				}
				if (Comp) {
					$$renderer.push("<!--[-->");
					Comp($$renderer, {
						element,
						$$slots: { element: true }
					});
					$$renderer.push("<!--]-->");
				} else {
					$$renderer.push("<!--[!-->");
					$$renderer.push("<!--]-->");
				}
			}
		}, true);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
