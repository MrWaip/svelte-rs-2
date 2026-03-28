import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Store {
		#data = $.state(null);
		get data() {
			return $.get(this.#data);
		}
		set data(value) {
			$.set(this.#data, value);
		}
		update(val) {
			this.data = val;
		}
	}
	let s = new Store();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, s.data));
	$.append($$anchor, p);
	$.pop();
}
