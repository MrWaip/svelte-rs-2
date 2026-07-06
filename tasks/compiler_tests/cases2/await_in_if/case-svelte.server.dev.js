App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = true;
		const promise = fetch("/api");
		if (show) {
			$$renderer.push("<!--[0-->");
			$.await($$renderer, promise, () => {}, (value) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 8, 2);
				$$renderer.push(`${$.escape(value)}</p>`);
				$.pop_element();
			});
			$$renderer.push(`<!--]-->`);
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
