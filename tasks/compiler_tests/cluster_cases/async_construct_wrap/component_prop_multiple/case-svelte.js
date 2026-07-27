import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, void 0, [() => delay($.get(x)), () => delay($.get(x) + 1)], ($$anchor, $0, $1) => {
		Child(node, {
			get a() {
				return $.get($0);
			},
			get b() {
				return $.get($1);
			}
		});
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
