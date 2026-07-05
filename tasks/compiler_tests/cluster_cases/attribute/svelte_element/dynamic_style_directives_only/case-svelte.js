import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <button>toggle</button>`, 1);
export default function App($$anchor) {
	let tag = "div";
	let value = $.state("red");
	const getValue = () => $.get(value);
	var fragment = root();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, ($0) => ({
			style: "",
			[$.STYLE]: $0
		}), [() => ({ color: getValue() })]);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.set(value, "blue"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
