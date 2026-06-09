import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>ok</p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	class Counter {}
	var $$exports = {
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
