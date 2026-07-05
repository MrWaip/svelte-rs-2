App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
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
			this.#items = $.tag($.state($.proxy([])), "Todo.items");
			this.#name = $.tag($.state(""), "Todo.name");
		}
		add() {
			this.items.push(this.name);
		}
	}
	let todo = new Todo();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${todo.items.length ?? ""} - ${todo.name ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
