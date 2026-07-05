App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Store {
		#data = $.tag($.state($.proxy({ n: 0 })), "Store.#data");
		get data() {
			return $.get(this.#data);
		}
		inc() {
			$.get(this.#data).n++;
		}
	}
	const store = new Store();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, store.data.n));
	$.delegated("click", button, function click() {
		return store.inc();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
