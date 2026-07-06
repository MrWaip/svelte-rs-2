App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let cls = $$props["cls"];
		$$renderer.push(`<my-element${$.attr_class($.clsx(cls))}>`);
		$.push_element($$renderer, "my-element", 4, 0);
		$$renderer.push(`</my-element>`);
		$.pop_element();
		$.bind_props($$props, { cls });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
