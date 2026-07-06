App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const title = "world";
		$$renderer.push(`<!---->world <div>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`<br/>`);
		$.push_element($$renderer, "br", 8, 4);
		$.pop_element();
		$$renderer.push(` world</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 12, 0);
		$$renderer.push(`world</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
