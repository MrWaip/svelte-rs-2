import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded, x;
	var $$promises = $.run([async () => loaded = await delay(1), () => x = $.state(0)]);
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [
		$$promises[0],
		$$promises[0],
		$$promises[1]
	], [() => delay($.get(x))], ($$anchor, $0) => {
		Child(node, {
			get a() {
				return loaded;
			},
			get b() {
				return $.get($0);
			}
		});
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
