import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Store {
		#data = $.state($.proxy({ n: 0 }));
		get data() {
			return $.get(this.#data);
		}
		inc() {
			$.get(this.#data).n++;
		}
	}
	const store = new Store();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, store.data.n));
	$.delegated("click", button, () => store.inc());
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
