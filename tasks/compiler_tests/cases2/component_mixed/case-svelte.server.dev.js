App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
import Icon from "./Icon.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 6, 0);
		$$renderer.push(`Title</h1>`);
		$.pop_element();
		$$renderer.push(` `);
		Button($$renderer, {});
		$$renderer.push(`<!----> `);
		Icon($$renderer, {});
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
