App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let refs = [];
		const Derived_1 = $.derived(() => Widget);
		if (Derived_1()) {
			$$renderer.push("<!--[-->");
			Derived_1()($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
		$$renderer.push(` `);
		if (true) {
			$$renderer.push("<!--[0-->");
			const Const_0 = Widget;
			if (Const_0) {
				$$renderer.push("<!--[-->");
				Const_0($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
