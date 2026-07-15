App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <video></video> <input/> <input/> <input/> <input/> <details></details>`, 3), App[$.FILENAME], [
	[14, 0],
	[15, 0],
	[16, 0],
	[17, 0],
	[19, 0],
	[20, 0],
	[21, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rest = {};
	let w = $.tag($.state(0), "w");
	let rect = $.tag($.state(void 0), "rect");
	let time = $.tag($.state(0), "time");
	let ind = $.tag($.state(false), "ind");
	let files = $.tag($.state(void 0), "files");
	let el;
	let val = $.tag($.state(""), "val");
	let checked = $.tag($.state(false), "checked");
	let open = $.tag($.state(false), "open");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	$.attribute_effect(div, () => ({ ...rest }));
	$.bind_this(div, ($$value) => el = $$value, () => el);
	var video = $.sibling(div, 2);
	$.attribute_effect(video, () => ({ ...rest }));
	var input = $.sibling(video, 2);
	$.attribute_effect(input, () => ({
		type: "checkbox",
		...rest
	}), void 0, void 0, void 0, void 0, true);
	var input_1 = $.sibling(input, 2);
	$.attribute_effect(input_1, () => ({
		type: "file",
		...rest
	}), void 0, void 0, void 0, void 0, true);
	var input_2 = $.sibling(input_1, 2);
	$.attribute_effect(input_2, () => ({ ...rest }), void 0, void 0, void 0, void 0, true);
	var input_3 = $.sibling(input_2, 2);
	$.attribute_effect(input_3, () => ({
		type: "checkbox",
		...rest
	}), void 0, void 0, void 0, void 0, true);
	var details = $.sibling(input_3, 2);
	$.attribute_effect(details, () => ({ ...rest }));
	$.bind_element_size(div, "clientWidth", function set($$value) {
		$.set(w, $$value);
	});
	$.bind_resize_observer(div, "contentRect", function set($$value) {
		$.set(rect, $$value);
	});
	$.bind_current_time(video, function get() {
		return $.get(time);
	}, function set($$value) {
		$.set(time, $$value);
	});
	$.bind_property("indeterminate", "change", input, function set($$value) {
		$.set(ind, $$value);
	}, function get() {
		return $.get(ind);
	});
	$.bind_files(input_1, function get() {
		return $.get(files);
	}, function set($$value) {
		$.set(files, $$value);
	});
	$.bind_value(input_2, function get() {
		return $.get(val);
	}, function set($$value) {
		$.set(val, $$value);
	});
	$.bind_checked(input_3, function get() {
		return $.get(checked);
	}, function set($$value) {
		$.set(checked, $$value);
	});
	$.bind_property("open", "toggle", details, function set($$value) {
		$.set(open, $$value);
	}, function get() {
		return $.get(open);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
