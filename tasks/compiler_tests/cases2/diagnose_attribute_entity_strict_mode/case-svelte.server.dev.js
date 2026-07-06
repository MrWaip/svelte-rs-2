App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div data-a="&amp;amp=q &lt; ">`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`a</div>`);
		$.pop_element();
		$$renderer.push(` <div data-b="© &amp;reg=x > foo">`);
		$.push_element($$renderer, "div", 2, 0);
		$$renderer.push(`b</div>`);
		$.pop_element();
		$$renderer.push(` <div data-c="&amp;ok &amp;=q">`);
		$.push_element($$renderer, "div", 3, 0);
		$$renderer.push(`c</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
