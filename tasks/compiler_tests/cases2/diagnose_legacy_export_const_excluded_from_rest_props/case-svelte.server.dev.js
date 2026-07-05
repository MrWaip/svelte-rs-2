App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["api", "a"]);
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		const api = () => 1;
		const rest = $$restProps;
		$.bind_props($$props, {
			a,
			api
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
