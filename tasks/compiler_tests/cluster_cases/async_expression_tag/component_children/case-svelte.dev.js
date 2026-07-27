import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[11, 0]]);
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
	$.add_svelte_meta(() => Child(node, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text();
			$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss(delay($.get(x))))()]);
			$.append($$anchor, text);
		}),
		$$slots: { default: true }
	}), "component", App, 13, 0, { componentTag: "Child" });
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
