import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $obj = () => ($.validate_store(obj, "obj"), $.store_get(obj, "$obj", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	function go() {
		$.store_mutate(obj, $.untrack($obj).x = 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x += 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x ??= 5, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x++, $.untrack($obj));
		$.store_mutate(obj, ++$.untrack($obj).x, $.untrack($obj));
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
