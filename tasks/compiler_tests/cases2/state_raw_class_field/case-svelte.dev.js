App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Store {
		#items = $.tag($.state([]), "Store.items");
		get items() {
			return $.get(this.#items);
		}
		set items(value) {
			$.set(this.#items, value);
		}
	}
	let s = new Store();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, s.items));
	$.append($$anchor, p);
	return $.pop($$exports);
}
