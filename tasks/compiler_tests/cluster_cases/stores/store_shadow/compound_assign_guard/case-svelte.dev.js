import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>add</button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store($.get(count), "count"), $.store_get($.get(count), "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.tag($.mutable_source(0), "count");
	$.legacy_pre_effect(() => {}, () => {
		$.store_set($.get(count), 1);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return $.store_unsub($.set(count, $.get(count) + 2), "$count", $$stores);
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
