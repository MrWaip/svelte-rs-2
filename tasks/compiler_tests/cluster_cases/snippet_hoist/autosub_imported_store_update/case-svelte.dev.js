App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./store_mod.js";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const foo = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var button = root();
		$.delegated("click", button, function click() {
			return $.update_store(count, $count());
		});
		$.append($$anchor, button);
	});
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => foo($$anchor), "render", App, 9, 0);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
