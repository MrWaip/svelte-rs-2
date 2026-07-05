import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button>toggle</button>`, 1);
export default function App($$anchor) {
	let value = $.state("red");
	const getClass = () => $.get(value) === "blue";
	const getValue = () => $.get(value);
	var fragment = root();
	var div = $.first_child(fragment);
	let classes;
	let styles;
	var button = $.sibling(div, 2);
	$.template_effect(($0, $1) => {
		classes = $.set_class(div, 1, "", null, classes, $0);
		styles = $.set_style(div, "", styles, $1);
	}, [() => ({ blue: getClass() }), () => ({ color: getValue() })]);
	$.delegated("click", button, () => $.set(value, "blue"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
