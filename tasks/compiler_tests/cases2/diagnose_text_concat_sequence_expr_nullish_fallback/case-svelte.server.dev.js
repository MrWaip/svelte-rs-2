App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { Kind } from "./kinds";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 6, 0);
		$$renderer.push(`Prefix ${$.escape(item?.kind === Kind.A ? "one" : "two")} suffix</span>`);
		$.pop_element();
		$.bind_props($$props, { item });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
