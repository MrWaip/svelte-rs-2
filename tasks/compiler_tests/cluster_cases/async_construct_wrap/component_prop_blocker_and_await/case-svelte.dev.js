import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded, x;
	var $$promises = $.run([async () => loaded = (await $.track_reactivity_loss(delay(1)))(), () => x = $.tag($.state(0), "x")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [
		$$promises[0],
		$$promises[0],
		$$promises[1]
	], [async () => (await $.track_reactivity_loss(delay($.get(x))))()], ($$anchor, $0) => {
		$.add_svelte_meta(() => Child(node, {
			get a() {
				return loaded;
			},
			get b() {
				return $.get($0);
			}
		}), "component", App, 14, 0, { componentTag: "Child" });
	});
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
