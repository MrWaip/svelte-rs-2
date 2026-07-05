App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 1]]);
var root_1 = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let num = $.tag($.state(0), "num");
	function inc() {
		$.update(num);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.await(node, () => $$props.object, null, ($$anchor, $$source) => {
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
	}), "await", App, 9, 0);
	$.delegated("click", button, inc);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
