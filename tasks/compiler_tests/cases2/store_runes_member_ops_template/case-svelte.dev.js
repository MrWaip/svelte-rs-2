App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.add_locations($.from_html(`<button>set</button> <button>+=</button> <button>??=</button> <button>++</button> <button>--pre</button>`, 1), App[$.FILENAME], [
	[4, 0],
	[5, 0],
	[6, 0],
	[7, 0],
	[8, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $obj = () => ($.validate_store(obj, "obj"), $.store_get(obj, "$obj", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	var button_3 = $.sibling(button_2, 2);
	var button_4 = $.sibling(button_3, 2);
	$.delegated("click", button, function click() {
		return $.store_mutate(obj, $.untrack($obj).x = 1, $.untrack($obj));
	});
	$.delegated("click", button_1, function click_1() {
		return $.store_mutate(obj, $.untrack($obj).x += 1, $.untrack($obj));
	});
	$.delegated("click", button_2, function click_2() {
		return $.store_mutate(obj, $.untrack($obj).x ??= 1, $.untrack($obj));
	});
	$.delegated("click", button_3, function click_3() {
		return $.store_mutate(obj, $.untrack($obj).x++, $.untrack($obj));
	});
	$.delegated("click", button_4, function click_4() {
		return $.store_mutate(obj, --$.untrack($obj).x, $.untrack($obj));
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
