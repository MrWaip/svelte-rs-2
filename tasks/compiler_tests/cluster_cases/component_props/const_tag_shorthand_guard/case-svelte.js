import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<!> <button>b</button>`, 1);
export default function App($$anchor) {
	const row = ($$anchor) => {
		const kLit = $.derived(() => "x");
		const kLive = $.derived(() => $.get(live) + 1);
		const kCall = $.derived(() => Math.random());
		Child($$anchor, {
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
		});
	};
	let live = $.state(0);
	let plain = 7;
	function bump() {
		$.update(live);
	}
	var fragment_1 = root();
	var node = $.first_child(fragment_1);
	row(node);
	var button = $.sibling(node, 2);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment_1);
}
$.delegate(["click"]);
