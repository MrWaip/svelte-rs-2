App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { props } = $$props;
		$$renderer.select({
			value: "dog",
			...props
		}, ($$renderer) => {
			$$renderer.option({ value: "dog" }, ($$renderer) => {
				$.push_element($$renderer, "option", 6, 1);
				$$renderer.push(`Dog`);
				$.pop_element();
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
