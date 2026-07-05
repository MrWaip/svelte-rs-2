import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <button>toggle</button>`, 1);
export default function App($$anchor) {
	let tag = "div";
	let flag = $.state(false);
	var fragment = root();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			class: "",
			style: "",
			[$.CLASS]: { blue: $.get(flag) },
			[$.STYLE]: { color: $.get(flag) ? "red" : "blue" }
		}));
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.set(flag, !$.get(flag)));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
