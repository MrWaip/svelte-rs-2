App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Store {
		#data = $.tag($.state(null), "Store.data");
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
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, s.data));
	$.append($$anchor, p);
	return $.pop($$exports);
}
