App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = "t";
		let counter = 0;
		let flag = "x";
		function bump() {
			title = title + "!";
			counter += 1;
			flag = flag + "!";
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 13, 0);
		$$renderer.push(`bump</button>`);
		$.pop_element();
		$$renderer.push(` <div${$.attr("title", title)}${$.attr("data-counter", counter)}${$.attr("data-flag", flag)}>`);
		$.push_element($$renderer, "div", 14, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
