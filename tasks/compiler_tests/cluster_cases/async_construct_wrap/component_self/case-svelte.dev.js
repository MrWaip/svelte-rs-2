import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	let go = false;
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.async(node_1, void 0, [async () => (await $.track_reactivity_loss(delay($.get(x))))()], ($$anchor, $0) => {
				$.add_svelte_meta(() => App(node_1, { get value() {
					return $.get($0);
				} }), "component", App, 13, 1, { componentTag: "svelte:self" });
			});
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (go) $$render(consequent);
		}), "if", App, 12, 0);
	}
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
