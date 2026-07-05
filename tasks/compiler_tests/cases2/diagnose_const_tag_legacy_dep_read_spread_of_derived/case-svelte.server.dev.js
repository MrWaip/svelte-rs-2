App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let cond = $.fallback($$props["cond"], false);
		if (cond) {
			$$renderer.push("<!--[0-->");
			const xs = [cond ? 1 : 2];
			const ys = [3];
			const all = [...xs, ...ys];
			$$renderer.push(`${$.escape(all.length)}`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { cond });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
