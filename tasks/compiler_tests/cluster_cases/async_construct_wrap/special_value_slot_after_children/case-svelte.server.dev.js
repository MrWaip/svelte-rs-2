import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save(Promise.resolve("dog")))();
			$$renderer.select({ value: $$0 }, ($$renderer) => {
				$$renderer.option({}, ($$renderer) => {
					$.push_element($$renderer, "option", 2, 1);
					$$renderer.push(`--Please choose an option--`);
					$.pop_element();
				});
				$$renderer.child(async ($$renderer) => {
					const $$0 = (await $.save(Promise.resolve("dog")))();
					$$renderer.option({}, $$0);
				});
				$$renderer.option({}, ($$renderer) => {
					$.push_element($$renderer, "option", 4, 1);
					$$renderer.push(`cat`);
					$.pop_element();
				});
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
