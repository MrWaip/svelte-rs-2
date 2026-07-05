App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $.fallback($$props["a"], 0);
		let b = $.fallback($$props["b"], 0);
		let flag = $.fallback($$props["flag"], false);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {
			sum: a + b,
			neg: !flag,
			both: a && b,
			cond: flag ? a : b
		}, null);
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			a,
			b,
			flag
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
