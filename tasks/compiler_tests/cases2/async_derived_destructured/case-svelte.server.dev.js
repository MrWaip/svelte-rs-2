import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let url = "/api";
		var data, meta;
		var $$promises = $$renderer.run([async () => {
			var $$d = await $.async_derived(() => fetch(url).then((r) => r.json()));
			data = $.derived(() => $$d().data);
			meta = $.derived(() => $$d().meta);
		}]);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data())));
		$$renderer.push(`-`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(meta())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
