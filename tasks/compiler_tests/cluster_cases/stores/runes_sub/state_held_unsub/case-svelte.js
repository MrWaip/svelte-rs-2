import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button>set new store</button> <button>incr</button> <pre> </pre>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $store = () => $.store_get($.get(store), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.state(void 0);
	function setStore() {
		$.store_unsub($.set(store, writable(0), true), "$store", $$stores);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var pre = $.sibling(button_1, 2);
	var text = $.child(pre, true);
	$.reset(pre);
	$.template_effect(() => $.set_text(text, $store()));
	$.delegated("click", button, setStore);
	$.delegated("click", button_1, () => $.update_store($.get(store), $store()));
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
