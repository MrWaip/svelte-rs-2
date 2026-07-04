App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<button>set</button> <button>+=</button> <button>??=</button> <button>&&=</button> <button>||=</button> <button>++</button> <button>--pre</button>`, 1), App[$.FILENAME], [
	[4, 0],
	[5, 0],
	[6, 0],
	[7, 0],
	[8, 0],
	[9, 0],
	[10, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	var button_3 = $.sibling(button_2, 2);
	var button_4 = $.sibling(button_3, 2);
	var button_5 = $.sibling(button_4, 2);
	var button_6 = $.sibling(button_5, 2);
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
		return $.store_set(count, $count() && 5);
	});
	$.delegated("click", button_4, function click_4() {
		return $.store_set(count, $count() || 5);
	});
	$.delegated("click", button_5, function click_5() {
		return $.update_store(count, $count());
	});
	$.delegated("click", button_6, function click_6() {
		return $.update_pre_store(count, $count(), -1);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
