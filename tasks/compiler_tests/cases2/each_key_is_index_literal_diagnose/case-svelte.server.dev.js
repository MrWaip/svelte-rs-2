App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const facts = [
			"Cats have five toes on their front paws, but only four on the back.",
			"A group of flamingos is called a 'flamboyance'.",
			"Bananas are berries, but strawberries aren't."
		];
		$$renderer.push(`<ol>`);
		$.push_element($$renderer, "ol", 9, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(facts);
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let fact = each_array[i];
			$$renderer.push(`<li>`);
			$.push_element($$renderer, "li", 11, 2);
			$$renderer.push(`${$.escape(fact)}</li>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></ol>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
