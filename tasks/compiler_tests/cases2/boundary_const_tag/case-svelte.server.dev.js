App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = ["a", "b"];
		$$renderer.push(`<!--[-->`);
		{
			const count = items.length;
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 7, 1);
			$$renderer.push(`Count: ${$.escape(count)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
