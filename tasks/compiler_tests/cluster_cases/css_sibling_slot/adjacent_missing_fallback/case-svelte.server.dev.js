App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<x class="svelte-1schprl">`);
		$.push_element($$renderer, "x", 1, 0);
		$$renderer.push(`</x>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		$.slot($$renderer, $$props, "default", {}, () => {
			$$renderer.push(`<y>`);
			$.push_element($$renderer, "y", 4, 1);
			$$renderer.push(`fallback content</y>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]--> <z class="svelte-1schprl">`);
		$.push_element($$renderer, "z", 7, 0);
		$$renderer.push(`this should be green if the slot fallback is not rendered</z>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
