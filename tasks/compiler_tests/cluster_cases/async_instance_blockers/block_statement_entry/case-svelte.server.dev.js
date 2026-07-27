import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var data, y;
		var $$promises = $$renderer.run([async () => data = await fetch("/a"), () => {
			{
				console.log(1);
				console.log(2);
			}
			y = 1;
		}]);
		$$renderer.push(`<!---->`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data)));
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(y)));
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
