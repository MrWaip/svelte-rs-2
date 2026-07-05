import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
var root_1 = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor, $$props) {
	let num = $.state(0);
	function inc() {
		$.update(num);
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.await(node, () => $$props.object, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { v = $.get(num) } = $.get($$source);
			return { v };
		});
		var v = $.derived(() => $.get($$value).v);
		var button_1 = root();
		var text = $.child(button_1);
		$.reset(button_1);
		$.template_effect(() => $.set_text(text, `${$.get(v) ?? ""} ${$.get(num) ?? ""}`));
		$.append($$anchor, button_1);
	});
	$.delegated("click", button, inc);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
