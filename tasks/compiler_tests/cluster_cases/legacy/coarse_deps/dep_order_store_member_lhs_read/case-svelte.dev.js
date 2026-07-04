import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import { foo } from "lib";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $obj = () => ($.validate_store(obj, "obj"), $.store_get(obj, "$obj", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const obj = writable({});
	let c = $.tag($.mutable_source(""), "c");
	$.legacy_pre_effect(() => ($obj(), $.get(c), foo), () => {
		$.store_mutate(obj, $.untrack($obj).purpose = ($.get(c) ? $.get(c) : "") + foo({ type: $obj().type }), $.untrack($obj));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, function get() {
		return $.get(c);
	}, function set($$value) {
		$.set(c, $$value);
	});
	$.append($$anchor, input);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
