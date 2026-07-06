App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { numbers } from "./data.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function add() {
			numbers[numbers.length] = numbers.length + 1;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.push(`${$.escape(numbers.join(" + "))}</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`add</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
