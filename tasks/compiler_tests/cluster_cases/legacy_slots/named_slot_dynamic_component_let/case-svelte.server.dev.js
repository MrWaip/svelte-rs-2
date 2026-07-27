App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Parent from "./Parent.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let component = $$props["component"];
		Parent($$renderer, { $$slots: { item: ($$renderer, { item, index }) => {
			if (component) {
				$$renderer.push("<!--[-->");
				component($$renderer, {
					slot: "item",
					item,
					index
				});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		} } });
		$.bind_props($$props, { component });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
