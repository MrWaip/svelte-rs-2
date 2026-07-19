App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<h1 class="svelte-142nm4m">`);
		$.push_element($$renderer, "h1", 1, 0);
		$$renderer.push(`test</h1>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		$.slot($$renderer, $$props, "a", {}, null);
		$$renderer.push(`<!--]--> <!--[-->`);
		$.slot($$renderer, $$props, "b", {}, null);
		$$renderer.push(`<!--]--> <!--[-->`);
		$.slot($$renderer, $$props, "c", {}, null);
		$$renderer.push(`<!--]--> <span class="svelte-142nm4m">`);
		$.push_element($$renderer, "span", 5, 0);
		$$renderer.push(`Hello</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
