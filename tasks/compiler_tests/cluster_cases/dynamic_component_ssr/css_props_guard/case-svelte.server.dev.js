App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
import B from "./B.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = 0;
		let Comp = $.derived(() => value % 2 === 0 ? A : B);
		$.css_props($$renderer, true, { "--prop": "red" }, () => {
			if (Comp()) {
				$$renderer.push("<!--[-->");
				Comp()($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}, true);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
