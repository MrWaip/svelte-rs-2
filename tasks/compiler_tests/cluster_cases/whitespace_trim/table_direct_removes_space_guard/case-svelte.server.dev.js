App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<table>`);
		$.push_element($$renderer, "table", 1, 0);
		$$renderer.push(`<tbody>`);
		$.push_element($$renderer, "tbody", 1, 7);
		$$renderer.push(`<tr>`);
		$.push_element($$renderer, "tr", 2, 1);
		$$renderer.push(`<td>`);
		$.push_element($$renderer, "td", 2, 5);
		$$renderer.push(`a</td>`);
		$.pop_element();
		$$renderer.push(`</tr>`);
		$.pop_element();
		$$renderer.push(`<tr>`);
		$.push_element($$renderer, "tr", 3, 1);
		$$renderer.push(`<td>`);
		$.push_element($$renderer, "td", 3, 5);
		$$renderer.push(`b</td>`);
		$.pop_element();
		$$renderer.push(`</tr>`);
		$.pop_element();
		$$renderer.push(`</tbody>`);
		$.pop_element();
		$$renderer.push(`</table>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
