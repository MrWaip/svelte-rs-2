App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		$$renderer.push(`<br/>`);
		$.push_element($$renderer, "br", 5, 0);
		$.pop_element();
		$$renderer.push(`${$.escape(a)}<button>`);
		$.push_element($$renderer, "button", 5, 8);
		$$renderer.push(`inc</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
