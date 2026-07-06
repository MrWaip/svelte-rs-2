App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let section = "Dashboard";
		section = "Settings";
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>App - ${$.escape(section)}</title>`);
			});
			$$renderer.push(`<meta charset="utf-8"/>`);
			$.push_element($$renderer, "meta", 7, 1);
			$.pop_element();
			$$renderer.push(` <meta name="description" content="A page"/>`);
			$.push_element($$renderer, "meta", 9, 1);
			$.pop_element();
			$$renderer.push(` <link rel="stylesheet" href="/styles.css"/>`);
			$.push_element($$renderer, "link", 10, 1);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
