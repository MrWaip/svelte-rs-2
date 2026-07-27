App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rest = {};
		function onclick() {}
		$$renderer.push(`<div${$.attributes({
			...rest,
			class: "x"
		})}>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <button class="y">`);
		$.push_element($$renderer, "button", 7, 0);
		$$renderer.push(`b</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
