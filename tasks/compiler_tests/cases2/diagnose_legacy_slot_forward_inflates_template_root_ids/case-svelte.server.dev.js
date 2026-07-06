App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Wrap($$renderer, { $$slots: { action: ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			$.slot($$renderer, $$props, "action", {}, () => {
				$$renderer.push(`<div class="action svelte-1nvkc8o">`);
				$.push_element($$renderer, "div", 3, 8);
				$$renderer.push(`fallback</div>`);
				$.pop_element();
			});
			$$renderer.push(`<!--]-->`);
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
