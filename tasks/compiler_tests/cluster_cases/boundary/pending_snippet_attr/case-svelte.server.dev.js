import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(pending);
function pending($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<!---->loading`);
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		if (pending) {
			$$renderer.push(`<!--[!-->`);
			pending($$renderer);
			$$renderer.push(`<!--]-->`);
		} else {
			$$renderer.push(`<!--[-->`);
			{
				let data;
				var promises = $$renderer.run([async () => data = (await $.save(Promise.resolve("d")))()]);
				$$renderer.push(`<!---->`);
				$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data)));
			}
			$$renderer.push(`<!--]-->`);
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
