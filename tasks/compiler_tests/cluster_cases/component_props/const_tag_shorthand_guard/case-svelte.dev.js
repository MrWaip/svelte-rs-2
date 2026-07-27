App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<!> <button>b</button>`, 1), App[$.FILENAME], [[18, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		const kLit = $.tag($.derived(() => "x"), "kLit");
		$.get(kLit);
		const kLive = $.tag($.derived(() => $.get(live) + 1), "kLive");
		$.get(kLive);
		const kCall = $.tag($.derived(() => Math.random()), "kCall");
		$.get(kCall);
		$.add_svelte_meta(() => Child($$anchor, {
			get kLive() {
				return $.get(kLive);
			},
			get kCall() {
				return $.get(kCall);
			},
			plain,
			eLit: $.get(kLit),
			get eLive() {
				return $.get(kLive);
			}
		}), "component", App, 14, 1, { componentTag: "Child" });
	});
	let live = $.tag($.state(0), "live");
	let plain = 7;
	function bump() {
		$.update(live);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment_1 = root();
	var node = $.first_child(fragment_1);
	$.add_svelte_meta(() => row(node), "render", App, 17, 0);
	var button = $.sibling(node, 2);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment_1);
	return $.pop($$exports);
}
$.delegate(["click"]);
