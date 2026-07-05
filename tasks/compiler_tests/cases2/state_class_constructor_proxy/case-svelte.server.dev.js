App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Todo {
			constructor() {
				this.items = [];
				this.name = "";
			}
			add() {
				this.items.push(this.name);
			}
		}
		let todo = new Todo();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 14, 0);
		$$renderer.push(`${$.escape(todo.items.length)} - ${$.escape(todo.name)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
