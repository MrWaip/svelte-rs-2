import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>ok</p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	class Counter {}
	var $$exports = {
		...$.legacy_api(),
		get Counter() {
			return Counter;
		},
		set Counter($$value) {
			Counter = $$value;
		}
	};
	var p = root();
	$.append($$anchor, p);
	$.bind_prop($$props, "Counter", Counter);
	return $.pop($$exports);
}
