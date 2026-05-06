import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $total = () => ($.validate_store(total, "total"), $.store_get(total, "$total", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let total = writable(0);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $total()));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
