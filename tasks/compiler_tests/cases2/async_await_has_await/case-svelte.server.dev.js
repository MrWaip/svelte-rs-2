import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function getPromise() {
			return fetch("/api");
		}
		$$renderer.child_block(async ($$renderer) => {
			$.await($$renderer, (async () => (await $.save(getPromise()))())(), () => {}, (value) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 8, 1);
				$$renderer.push(`${$.escape(value)}</p>`);
				$.pop_element();
			});
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
