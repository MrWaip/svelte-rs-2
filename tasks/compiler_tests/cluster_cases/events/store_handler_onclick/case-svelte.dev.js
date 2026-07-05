App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $handler = () => ($.validate_store(handler, "handler"), $.store_get(handler, "$handler", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const handler = writable();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.apply($handler, this, $$args, App, [6, 17]);
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
