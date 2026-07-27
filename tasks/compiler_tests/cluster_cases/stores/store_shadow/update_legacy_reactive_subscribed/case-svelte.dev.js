import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>increment</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store($.get(count), "count"), $.store_get($.get(count), "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let base = $.prop($$props, "base", 8, 0);
	let count = $.tag($.mutable_source(0), "count");
	$.legacy_pre_effect(() => $.deep_read_state(base()), () => {
		$.store_unsub($.set(count, base() * 2), "$count", $$stores);
	});
	$.legacy_pre_effect(() => {}, () => {
		$.store_set($.get(count), 1);
	});
	$.legacy_pre_effect_reset();
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
