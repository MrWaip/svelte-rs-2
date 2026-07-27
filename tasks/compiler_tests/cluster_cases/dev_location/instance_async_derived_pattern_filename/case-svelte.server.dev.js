import "svelte/internal/flags/async";
App[$.FILENAME] = "src/lib/Widget.svelte";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { p } = $$props;
		var first, second;
		var $$promises = $$renderer.run([async () => {
			var $$d = await $.async_derived(() => p);
			first = $.derived(() => $$d().first);
			second = $.derived(() => $$d().second);
		}]);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(first())));
		$$renderer.push(` `);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(second())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
