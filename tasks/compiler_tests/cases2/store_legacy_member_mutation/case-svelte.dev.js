import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.add_locations($.from_html(`<button>++</button> <button>=</button>`, 1), App[$.FILENAME], [[4, 0], [5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $obj = () => ($.validate_store(obj, "obj"), $.store_get(obj, "$obj", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	$.delegated("click", button, function click() {
		return $.store_mutate(obj, $.untrack($obj).value++, $.untrack($obj));
	});
	$.delegated("click", button_1, function click_1() {
		return $.store_mutate(obj, $.untrack($obj).value = 1, $.untrack($obj));
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
