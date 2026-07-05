import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $y = () => ($.validate_store(y, "y"), $.store_get(y, "$y", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const y = writable(0);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $y()));
	$.bind_window_scroll("y", function get() {
		return $y();
	}, function set($$value) {
		$.store_set(y, $$value);
	});
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
