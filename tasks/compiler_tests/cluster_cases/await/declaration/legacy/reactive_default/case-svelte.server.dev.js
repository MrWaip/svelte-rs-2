App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let p = Promise.resolve({});
		let num = 0;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$.await($$renderer, p, () => {}, ({ v = num }) => {
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 8, 1);
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
