App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>+</button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var button = root();
			$.template_effect(() => {
				console.log({ count: $.snapshot($.get(count)) });
				debugger;
			});
			$.delegated("click", button, function click() {
				return $.update(count);
			});
			$.append($$anchor, button);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(count)) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
