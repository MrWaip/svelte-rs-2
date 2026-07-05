App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
import A from "./A.svelte";
import B from "./B.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let flag = $.fallback($$props["flag"], false);
		Inner($$renderer, { $$slots: { icon: ($$renderer) => {
			if (flag ? A : B) {
				$$renderer.push("<!--[-->");
				(flag ? A : B)($$renderer, { slot: "icon" });
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		} } });
		$.bind_props($$props, { flag });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
