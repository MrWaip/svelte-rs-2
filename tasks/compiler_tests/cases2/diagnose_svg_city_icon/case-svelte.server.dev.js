App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<svg viewBox="0 0 24 24" fill="none" width="24" height="24" xmlns="http://www.w3.org/2000/svg">`);
		$.push_element($$renderer, "svg", 5, 0);
		$$renderer.push(`<rect x="3" y="11" width="6" height="10" stroke="#0070f3" stroke-width="2">`);
		$.push_element($$renderer, "rect", 6, 1);
		$$renderer.push(`</rect>`);
		$.pop_element();
		$$renderer.push(`<rect x="9" y="7" width="6" height="14" stroke="#0070f3" stroke-width="2">`);
		$.push_element($$renderer, "rect", 7, 1);
		$$renderer.push(`</rect>`);
		$.pop_element();
		$$renderer.push(`<rect x="15" y="3" width="6" height="18" stroke="#0070f3" stroke-width="2">`);
		$.push_element($$renderer, "rect", 8, 1);
		$$renderer.push(`</rect>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
		// City icon
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
