App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const promise = fetch("/api");
		$.await($$renderer, promise, () => {
			$$renderer.push(`<div class="loading">`);
			$.push_element($$renderer, "div", 6, 1);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 2);
			$$renderer.push(`Please wait...</span>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
		}, (value) => {
			$$renderer.push(`<div class="result">`);
			$.push_element($$renderer, "div", 10, 1);
			$$renderer.push(`<h1>`);
			$.push_element($$renderer, "h1", 11, 2);
			$$renderer.push(`Result</h1>`);
			$.pop_element();
			$$renderer.push(` <p>`);
			$.push_element($$renderer, "p", 12, 2);
			$$renderer.push(`${$.escape(value)}</p>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
