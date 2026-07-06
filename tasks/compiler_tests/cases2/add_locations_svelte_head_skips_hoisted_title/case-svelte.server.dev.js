App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { title = "x" } = $$props;
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>${$.escape(title)}</title>`);
			});
			$$renderer.push(`<meta name="x" content="y"/>`);
			$.push_element($$renderer, "meta", 7, 4);
			$.pop_element();
			$$renderer.push(` <link rel="canonical" href="/"/>`);
			$.push_element($$renderer, "link", 8, 4);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
