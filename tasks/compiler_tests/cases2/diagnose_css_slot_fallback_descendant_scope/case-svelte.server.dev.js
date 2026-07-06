App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="icon-slot svelte-1qciquw">`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "icon", {}, () => {
			$$renderer.push(`<img alt="" class="svelte-1qciquw"/>`);
			$.push_element($$renderer, "img", 3, 8);
			$.pop_element();
		});
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
