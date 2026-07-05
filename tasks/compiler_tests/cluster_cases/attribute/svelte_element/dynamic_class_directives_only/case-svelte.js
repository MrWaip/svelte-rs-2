import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <button>toggle</button>`, 1);
export default function App($$anchor) {
	let tag = "div";
	let value = $.state("red");
	const getClass = () => $.get(value) === "blue";
	var fragment = root();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		let classes;
		$.template_effect(($0) => classes = $.set_class($$element, 0, "", null, classes, $0), [() => ({ blue: getClass() })]);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.set(value, "blue"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
