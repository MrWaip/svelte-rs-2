App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		{
			let dt = $.derived(() => [
				1,
				2,
				3
			]);
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 1, 0);
			$$renderer.push(`${$.escape(dt.length)}
	${$.escape(dt.map((x) => x + dt().length))}</div>`);
			$.pop_element();
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
