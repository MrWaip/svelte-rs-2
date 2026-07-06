App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		function handleScroll() {
			count++;
		}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>Count: ${$.escape(count)}</title>`);
			});
			$$renderer.push(`<meta name="description" content="test"/>`);
			$.push_element($$renderer, "meta", 11, 4);
			$.pop_element();
		});
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 16, 0);
		$$renderer.push(`Count: ${$.escape(count)}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
