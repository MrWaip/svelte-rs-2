import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<button>=</button> <button>+=</button> <button>??=</button> <button>++</button>`, 1), App[$.FILENAME], [
	[4, 0],
	[5, 0],
	[6, 0],
	[7, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	var button_3 = $.sibling(button_2, 2);
	$.delegated("click", button, function click() {
		return $.store_set(count, 1);
	});
	$.delegated("click", button_1, function click_1() {
		return $.store_set(count, $count() + 1);
	});
	$.delegated("click", button_2, function click_2() {
		return $.store_set(count, $count() ?? 5);
	});
	$.delegated("click", button_3, function click_3() {
		return $.update_store(count, $count());
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
