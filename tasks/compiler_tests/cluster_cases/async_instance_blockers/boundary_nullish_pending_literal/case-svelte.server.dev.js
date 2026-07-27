import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let pending = null;
		if (pending) {
			$$renderer.push(`<!--[!-->`);
			pending($$renderer);
			$$renderer.push(`<!--]-->`);
		} else {
			$$renderer.push(`<!--[-->`);
			{
				$$renderer.push(`<!---->`);
				$$renderer.push(async () => $.escape(await "awaited"));
			}
			$$renderer.push(`<!--]-->`);
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
