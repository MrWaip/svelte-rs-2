App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = 0;
		let bar = 0;
		foo = 4;
		$: {
			bar = 0;
			outer: for (let i = 0; i < foo; i++) {
				for (let j = 0; j < foo; j++) {
					if (j > i) break outer;
					bar += j;
				}
			}
		}
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 16, 0);
		$$renderer.push(`${$.escape(foo)} ${$.escape(bar)}</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
