App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let status = $.fallback($$props["status"], "neutral");
		function classify(s) {
			return s + "-x";
		}
		$$renderer.push(`<div${$.attr_class($.clsx(classify(status)))}>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { status });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
