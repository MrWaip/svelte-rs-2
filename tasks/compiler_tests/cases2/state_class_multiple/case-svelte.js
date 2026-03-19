import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Form {
		#name = $.state("");
		get name() {
			return $.get(this.#name);
		}
		set name(value) {
			$.set(this.#name, value, true);
		}
		#email = $.state("");
		get email() {
			return $.get(this.#email);
		}
		set email(value) {
			$.set(this.#email, value, true);
		}
		#data = $.state({});
		get data() {
			return $.get(this.#data);
		}
		set data(value) {
			$.set(this.#data, value);
		}
	}
	let f = new Form();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, f.name));
	$.append($$anchor, p);
	$.pop();
}
