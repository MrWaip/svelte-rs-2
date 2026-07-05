App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	$.store_set(count, 5);
	$.update_store(count, $count());
	$.update_pre_store(count, $count());
	$.update_store(count, $count(), -1);
	$.store_set(count, $count() + 10);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $count()));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
