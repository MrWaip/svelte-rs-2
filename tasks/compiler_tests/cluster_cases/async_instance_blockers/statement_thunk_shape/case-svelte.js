import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor) {
	let gate = $.state(0);
	let sink = 0;
	var loaded, single;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate)), () => {
		void (sink = 1);
		void (sink = 2);
		single = $.get(gate) + 1;
	}]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${sink ?? ""}${single}${$.get(loaded) ?? ""}`), void 0, void 0, [
		$$promises[1],
		$$promises[1],
		$$promises[0]
	]);
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
