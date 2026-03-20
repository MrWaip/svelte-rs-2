import * as $ from "svelte/internal/client";
import { store } from "./stores";
var root = $.from_html(`<button>set</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $store = () => $.store_get(store, "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	$.store_mutate(store, $.untrack($store).field = "hello", $.untrack($store));
	var button = root();
	$.delegated("click", button, () => $.store_mutate(store, $.untrack($store).field = "world", $.untrack($store)));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
