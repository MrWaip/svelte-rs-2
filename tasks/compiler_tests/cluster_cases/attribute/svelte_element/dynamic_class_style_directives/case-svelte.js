import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <button>toggle</button>`, 1);
export default function App($$anchor) {
	let tag = "div";
	let value = $.state("red");
	const getClass = () => $.get(value) === "blue";
	const getValue = () => $.get(value);
	var fragment = root();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, ($0, $1) => ({
			class: "",
			style: "",
			[$.CLASS]: $0,
			[$.STYLE]: $1
		}), [() => ({ blue: getClass() }), () => ({ color: getValue() })]);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.set(value, "blue"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
