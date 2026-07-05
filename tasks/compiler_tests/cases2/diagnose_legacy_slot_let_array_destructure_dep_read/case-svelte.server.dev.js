App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $.fallback($$props["a"], null);
		Inner($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { value: [a] }) => {
				const x = a ? a({ k: 1 }) : null;
				$$renderer.push(`<!---->${$.escape(x)}`);
			} }
		});
		$.bind_props($$props, { a });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
