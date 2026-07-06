App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let p = Promise.resolve([{ fav: false }]);
		$.await($$renderer, p, () => {}, (cards) => {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(cards.filter((c) => !c.fav));
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let card = each_array[$$index];
				$$renderer.push(`<button>`);
				$.push_element($$renderer, "button", 8, 2);
				$$renderer.push(`${$.escape(card.fav)}</button>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
