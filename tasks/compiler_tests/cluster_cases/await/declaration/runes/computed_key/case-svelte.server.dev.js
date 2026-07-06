App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const k = "z";
		let p = Promise.resolve({ z: 1 });
		$.await($$renderer, p, () => {}, ({ [k]: v }) => {
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 6, 1);
			$$renderer.push(`${$.escape(v)}</button>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
