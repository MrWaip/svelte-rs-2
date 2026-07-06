import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
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
		$$renderer.push(`<p>${$.escape(todo.items.length)} - ${$.escape(todo.name)}</p>`);
	});
}
