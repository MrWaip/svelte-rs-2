App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let variant = "primary";
		$.prevent_snippet_stringification(badge);
		function badge($$renderer, text) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span${$.attr_class("badge svelte-yczv4j", void 0, { "primary": variant === "primary" })}>`);
			$.push_element($$renderer, "span", 6, 1);
			$$renderer.push(`${$.escape(text)}</span>`);
			$.pop_element();
		}
		badge($$renderer, "hi");
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
