App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let current = Inner;
		if (current) {
			$$renderer.push("<!--[-->");
			current($$renderer, { $$slots: { caption: ($$renderer) => {
				$$renderer.push(`<span slot="caption">`);
				$.push_element($$renderer, "span", 7, 4);
				$$renderer.push(`hi</span>`);
				$.pop_element();
			} } });
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
