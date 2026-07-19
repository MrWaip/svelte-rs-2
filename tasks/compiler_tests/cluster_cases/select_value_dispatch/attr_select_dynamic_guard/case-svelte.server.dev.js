App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let v = "dog";
		$$renderer.select({ value: v }, ($$renderer) => {
			$$renderer.option({ value: "dog" }, ($$renderer) => {
				$.push_element($$renderer, "option", 6, 1);
				$$renderer.push(`Dog`);
				$.pop_element();
			});
			$$renderer.option({ value: "cat" }, ($$renderer) => {
				$.push_element($$renderer, "option", 7, 1);
				$$renderer.push(`Cat`);
				$.pop_element();
			});
		});
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`swap</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
