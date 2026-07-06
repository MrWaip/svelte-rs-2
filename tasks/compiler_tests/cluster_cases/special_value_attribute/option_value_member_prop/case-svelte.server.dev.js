App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		$$renderer.option({ value: item.key }, ($$renderer) => {
			$.push_element($$renderer, "option", 6, 1);
			$$renderer.push(`${$.escape(item.name)}`);
			$.pop_element();
		});
		$$renderer.option({ value: "b" }, ($$renderer) => {
			$.push_element($$renderer, "option", 7, 1);
			$$renderer.push(`Two`);
			$.pop_element();
		});
		$$renderer.push(`</select>`);
		$.pop_element();
		$.bind_props($$props, { item });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
