App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<button>inc</button> <button>pre inc</button> <button>dec</button>`, 1), App[$.FILENAME], [
	[5, 0],
	[6, 0],
	[7, 0]
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
	$.delegated("click", button, function click() {
		return $.update_store(count, $count());
	});
	$.delegated("click", button_1, function click_1() {
		return $.update_pre_store(count, $count());
	});
	$.delegated("click", button_2, function click_2() {
		return $.update_store(count, $count(), -1);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
