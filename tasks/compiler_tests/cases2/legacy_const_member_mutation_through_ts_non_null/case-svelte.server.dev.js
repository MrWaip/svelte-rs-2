App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function loader() {
			return { data: { selected: null } };
		}
		const state = loader();
		function reset() {
			if (state.data === null) return;
			state.data.selected = null;
		}
		function pick(value) {
			state.data.selected = value;
		}
		if (state.data) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 23, 4);
			$$renderer.push(`${$.escape(state.data.selected)}</button>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 25, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
