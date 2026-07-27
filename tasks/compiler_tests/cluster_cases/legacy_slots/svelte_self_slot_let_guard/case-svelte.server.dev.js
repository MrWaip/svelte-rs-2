App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let depth = $.fallback($$props["depth"], 0);
		if (depth > 0) {
			$$renderer.push("<!--[0-->");
			App($$renderer, {
				depth: depth - 1,
				children: $.invalid_default_snippet,
				$$slots: { default: ($$renderer, { item, index }) => {
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 7, 2);
					$$renderer.push(`${$.escape(item)} ${$.escape(index)}</p>`);
					$.pop_element();
				} }
			});
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { depth });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
