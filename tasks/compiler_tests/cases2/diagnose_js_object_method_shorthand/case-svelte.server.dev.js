App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const obj = { run(x) {
			return x + 1;
		} };
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.push(`${$.escape(obj.run(1))}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
