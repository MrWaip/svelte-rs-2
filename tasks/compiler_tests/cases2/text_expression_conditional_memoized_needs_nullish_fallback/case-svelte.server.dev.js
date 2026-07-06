App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { card } = $$props;
		function fmt(x) {
			return x;
		}
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 5, 0);
		$$renderer.push(`${$.escape(card.flag ? "A" : "B" + fmt(card.x))} • ${$.escape(card.tail)}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
