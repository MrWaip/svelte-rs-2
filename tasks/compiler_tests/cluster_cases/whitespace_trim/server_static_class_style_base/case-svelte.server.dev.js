App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<span class="tsCompact300XSmall">`);
		$.push_element($$renderer, "span", 1, 0);
		$$renderer.push(`x</span>`);
		$.pop_element();
		$$renderer.push(` <div style="--a: b;">`);
		$.push_element($$renderer, "div", 2, 0);
		$$renderer.push(`y</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
