import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var name, age;
		var $$promises = $$renderer.run([async () => {
			var $$d = await $.async_derived(() => fetch("/api"));
			name = $.derived(() => $$d().name);
			age = $.derived(() => $$d().age);
		}]);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(name())));
		$$renderer.push(` `);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(age())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
