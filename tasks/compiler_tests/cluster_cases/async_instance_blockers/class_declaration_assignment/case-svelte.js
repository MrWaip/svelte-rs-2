import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let gate = $.state(0);
	var loaded, Box, box;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate)), () => {
		Box = class Box {
			value = 1;
		};
		box = new Box();
	}]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(loaded) ?? ""}${box.value ?? ""}`), void 0, void 0, [$$promises[0], $$promises[1]]);
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
