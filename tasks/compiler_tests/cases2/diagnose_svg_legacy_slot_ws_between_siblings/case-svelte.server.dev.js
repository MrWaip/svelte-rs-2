App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, { $$slots: { header: ($$renderer) => {
			{
				$$renderer.push(`<g>`);
				$.push_element($$renderer, "g", 7, 2);
				$$renderer.push(`<path d="M1">`);
				$.push_element($$renderer, "path", 7, 5);
				$$renderer.push(`</path>`);
				$.pop_element();
				$$renderer.push(`</g>`);
				$.pop_element();
				$$renderer.push(`<g>`);
				$.push_element($$renderer, "g", 8, 2);
				$$renderer.push(`<path d="M2">`);
				$.push_element($$renderer, "path", 8, 5);
				$$renderer.push(`</path>`);
				$.pop_element();
				$$renderer.push(`</g>`);
				$.pop_element();
			}
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
