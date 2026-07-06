App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { props } = $$props;
		if (true) {
			$$renderer.push("<!--[0-->");
			const { x } = props;
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 7, 2);
			$$renderer.push(`${$.escape(x)}</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
