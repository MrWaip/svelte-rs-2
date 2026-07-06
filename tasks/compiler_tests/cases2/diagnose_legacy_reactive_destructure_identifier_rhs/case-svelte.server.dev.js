App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a, b;
		function makePair() {
			return [1, 2];
		}
		const pair = makePair();
		$: [a, b] = pair;
		$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
