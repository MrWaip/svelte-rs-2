App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $$props["obj"];
		$$renderer.push(`<details${$.attr("open", obj.flag, true)}>`);
		$.push_element($$renderer, "details", 4, 0);
		$$renderer.push(`<summary>`);
		$.push_element($$renderer, "summary", 4, 30);
		$$renderer.push(`x</summary>`);
		$.pop_element();
		$$renderer.push(`</details>`);
		$.pop_element();
		$.bind_props($$props, { obj });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
