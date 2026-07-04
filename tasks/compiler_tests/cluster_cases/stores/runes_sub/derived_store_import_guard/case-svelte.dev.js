App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable, derived } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $store = () => ($.validate_store(store, "store"), $.store_get(store, "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const store = writable(0);
	let doubled = $.tag($.derived(() => $store() * 2), "doubled");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(doubled) ?? ""} ${$store() ?? ""}`));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
