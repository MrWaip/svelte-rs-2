App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = 0;
		let bar;
		foo = 1;
		$: {
			bar = foo + 1;
			if (foo) {
				break $;
			}
			bar = foo + 2;
		}
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 14, 0);
		$$renderer.push(`${$.escape(foo)} ${$.escape(bar)}</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
