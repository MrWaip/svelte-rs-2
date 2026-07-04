App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <button>toggle</button>`, 1), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let flag = $.tag($.state(false), "flag");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			$.attribute_effect($$element, () => ({
				class: "",
				style: "",
				[$.CLASS]: { blue: $.get(flag) },
				[$.STYLE]: { color: $.get(flag) ? "red" : "blue" }
			}));
		}, void 0, [6, 0]);
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return $.set(flag, !$.get(flag));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
