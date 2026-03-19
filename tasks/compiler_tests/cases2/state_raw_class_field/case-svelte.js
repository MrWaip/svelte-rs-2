import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Store {
		#items = $.state([]);
		get items() {
			return $.get(this.#items);
		}
		set items(value) {
			$.set(this.#items, value);
		}
	}
	let s = new Store();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, s.items));
	$.append($$anchor, p);
	$.pop();
}
