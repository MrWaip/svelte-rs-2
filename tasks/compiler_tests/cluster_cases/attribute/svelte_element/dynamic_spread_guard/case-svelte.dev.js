App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <button>toggle</button>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let value = $.tag($.state("red"), "value");
	const getSpread = () => ({ class: $.get(value) });
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			$.attribute_effect($$element, ($0) => ({ ...$0 }), [() => getSpread()]);
		}, void 0, [7, 0]);
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return $.set(value, "blue");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
