App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		function tick() {
			count += 1;
			return count;
		}
		let show = true;
		if (show) {
			$$renderer.push("<!--[0-->");
			const value = tick();
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 8, 4);
			$$renderer.push(`${$.escape(value)}</p>`);
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
