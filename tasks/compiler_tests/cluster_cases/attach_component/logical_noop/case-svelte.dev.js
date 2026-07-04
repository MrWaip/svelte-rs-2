App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root = $.add_locations($.from_html(`<button></button> <!>`, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function attachment() {
		console.log("up");
	}
	let enabled = $.tag($.state(false), "enabled");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => Inner(node, { [$.attachment()]: ($$node) => ($.get(enabled) && attachment || $.noop)($$node) }), "component", App, 13, 0, { componentTag: "Inner" });
	$.delegated("click", button, function click() {
		return $.set(enabled, !$.get(enabled));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
