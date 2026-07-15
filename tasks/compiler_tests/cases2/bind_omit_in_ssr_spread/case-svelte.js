import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <video></video> <input/> <input/> <input/> <input/> <details></details>`, 3);
export default function App($$anchor) {
	let rest = {};
	let w = $.state(0);
	let rect = $.state(void 0);
	let time = $.state(0);
	let ind = $.state(false);
	let files = $.state(void 0);
	let el;
	let val = $.state("");
	let checked = $.state(false);
	let open = $.state(false);
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
	$.bind_element_size(div, "clientWidth", ($$value) => $.set(w, $$value));
	$.bind_resize_observer(div, "contentRect", ($$value) => $.set(rect, $$value));
	$.bind_current_time(video, () => $.get(time), ($$value) => $.set(time, $$value));
	$.bind_property("indeterminate", "change", input, ($$value) => $.set(ind, $$value), () => $.get(ind));
	$.bind_files(input_1, () => $.get(files), ($$value) => $.set(files, $$value));
	$.bind_value(input_2, () => $.get(val), ($$value) => $.set(val, $$value));
	$.bind_checked(input_3, () => $.get(checked), ($$value) => $.set(checked, $$value));
	$.bind_property("open", "toggle", details, ($$value) => $.set(open, $$value), () => $.get(open));
	$.append($$anchor, fragment);
}
