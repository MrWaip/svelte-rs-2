import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Todo {
		#items;
		get items() {
			return $.get(this.#items);
		}
		set items(value) {
			$.set(this.#items, value, true);
		}
		#name;
		get name() {
			return $.get(this.#name);
		}
		set name(value) {
			$.set(this.#name, value, true);
		}
		constructor() {
			this.#items = $.state($.proxy([]));
			this.#name = $.state("");
		}
		add() {
			this.items.push(this.name);
		}
	}
	let todo = new Todo();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${todo.items.length ?? ""} - ${todo.name ?? ""}`));
	$.append($$anchor, p);
	$.pop();
}
