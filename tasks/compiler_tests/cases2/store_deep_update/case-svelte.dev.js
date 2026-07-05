App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { store } from "./stores";
var root = $.add_locations($.from_html(`<button>inc</button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $store = () => ($.validate_store(store, "store"), $.store_get(store, "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return $.store_mutate(store, $.untrack($store).count++, $.untrack($store));
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
