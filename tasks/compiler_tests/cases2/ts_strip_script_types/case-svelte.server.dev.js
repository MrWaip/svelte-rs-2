App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = "world";
		let status = "active";
		function greet(user) {
			return `Hello ${user.name}`;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 19, 0);
		$$renderer.push(`world</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 20, 0);
		$$renderer.push(`active</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
