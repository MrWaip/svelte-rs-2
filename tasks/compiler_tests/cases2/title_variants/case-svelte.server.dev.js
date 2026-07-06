App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let pageTitle = "Home";
		let section = "Dashboard";
		pageTitle = "Other";
		section = "Settings";
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>Static - ${$.escape(pageTitle)} - App ${$.escape(section)}</title>`);
			});
			$$renderer.push(`<meta name="description" content="test"/>`);
			$.push_element($$renderer, "meta", 10, 1);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
