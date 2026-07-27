App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>increment</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store($.get(count), "count"), $.store_get($.get(count), "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.tag($.state(0), "count");
	$.user_effect(() => {
		$.store_set($.get(count), 1);
	});
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
