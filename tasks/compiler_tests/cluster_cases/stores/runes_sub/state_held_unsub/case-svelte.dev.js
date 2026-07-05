App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button>set new store</button> <button>incr</button> <pre> </pre>`, 1), App[$.FILENAME], [
	[9, 0],
	[10, 0],
	[11, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $store = () => ($.validate_store($.get(store), "store"), $.store_get($.get(store), "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.tag($.state(void 0), "store");
	function setStore() {
		$.store_unsub($.set(store, writable(0), true), "$store", $$stores);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var pre = $.sibling(button_1, 2);
	var text = $.child(pre, true);
	$.reset(pre);
	$.template_effect(() => $.set_text(text, $store()));
	$.delegated("click", button, setStore);
	$.delegated("click", button_1, function click() {
		return $.update_store($.get(store), $store());
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
