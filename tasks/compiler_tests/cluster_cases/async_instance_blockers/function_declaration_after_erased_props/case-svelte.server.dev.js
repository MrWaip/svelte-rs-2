import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { p } = $$props;
		function f() {
			return 1;
		}
		var a, b;
		var $$promises = $$renderer.run([async () => a = await p, () => b = 2]);
		$$renderer.push(`<!---->`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(a)));
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b)));
		$$renderer.push(`${$.escape(f())}`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
