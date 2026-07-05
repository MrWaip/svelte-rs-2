App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data, details;
		const default_details = { country: "" };
		$: data = {
			locked: false,
			details: null
		};
		$: details = data.details ?? default_details;
		$$renderer.select({
			value: details.country,
			disabled: data.locked
		}, ($$renderer) => {
			$$renderer.option({ value: "" }, ($$renderer) => {
				$.push_element($$renderer, "option", 19, 1);
				$$renderer.push(`Select`);
				$.pop_element();
			});
			$$renderer.option({ value: "us" }, ($$renderer) => {
				$.push_element($$renderer, "option", 20, 1);
				$$renderer.push(`US`);
				$.pop_element();
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
