App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let flag = $.fallback($$props["flag"], false);
		let Comp = $$props["Comp"];
		function onA() {}
		function onB() {}
		if (Comp) {
			$$renderer.push("<!--[-->");
			Comp($$renderer, {
				onA: flag ? onA : undefined,
				onB: flag ? onB : undefined
			});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
		$.bind_props($$props, {
			flag,
			Comp
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
