import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const count = writable(0);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $count()));
	$.append($$anchor, div);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
