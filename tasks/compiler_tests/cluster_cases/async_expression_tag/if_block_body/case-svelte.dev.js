import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			var text = $.text();
			$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss(delay($.get(x))))()]);
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(x) >= 0) $$render(consequent);
		}), "if", App, 11, 0);
	}
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
