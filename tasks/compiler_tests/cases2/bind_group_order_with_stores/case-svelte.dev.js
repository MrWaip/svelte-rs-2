App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<input type="radio"/> <input type="radio"/> <p> </p>`, 1), App[$.FILENAME], [
	[8, 0],
	[9, 0],
	[10, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $metrics = () => ($.validate_store(metrics, "metrics"), $.store_get(metrics, "$metrics", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const binding_group = [];
	let metrics = writable([
		1,
		2,
		3
	]);
	let group = $.tag($.state($.proxy([])), "group");
	let total = $.tag($.derived(() => $metrics().length), "total");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = "b";
	var p = $.sibling(input_1, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(total)));
	$.bind_group(binding_group, [], input, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.bind_group(binding_group, [], input_1, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
