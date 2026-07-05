App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let val = "25";
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 6, 0);
		$.css_props($$renderer, true, { "--color": `px ${$.stringify(val)}` }, () => {
			Child($$renderer, {});
		});
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
