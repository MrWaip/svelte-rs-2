App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Form {
		#name = $.tag($.state(""), "Form.name");
		get name() {
			return $.get(this.#name);
		}
		set name(value) {
			$.set(this.#name, value, true);
		}
		#email = $.tag($.state(""), "Form.email");
		get email() {
			return $.get(this.#email);
		}
		set email(value) {
			$.set(this.#email, value, true);
		}
		#data = $.tag($.state({}), "Form.data");
		get data() {
			return $.get(this.#data);
		}
		set data(value) {
			$.set(this.#data, value);
		}
	}
	let f = new Form();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, f.name));
	$.append($$anchor, p);
	return $.pop($$exports);
}
