App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const outer = fetch("/api/list");
		$.await($$renderer, outer, () => {}, (items) => {
			$.await($$renderer, items[0], () => {}, (detail) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 7, 2);
				$$renderer.push(`${$.escape(detail)}</p>`);
				$.pop_element();
			});
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
