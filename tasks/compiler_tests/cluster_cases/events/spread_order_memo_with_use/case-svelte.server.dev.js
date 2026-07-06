App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rest = $.fallback($$props["rest"], () => ({}), true);
		const bubbler = createBubbler();
		function action(node) {}
		$$renderer.push(`<div${$.attributes({ ...rest })}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { rest });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
