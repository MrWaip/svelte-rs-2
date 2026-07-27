import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>toggle</button> <!> <p> </p>`, 1);
export default function App($$anchor) {
	let gate = $.state(true);
	var loaded;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate))]);
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [$$promises[0]], void 0, (node) => {
		var consequent = ($$anchor) => {
			var text = $.text("yes");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($.get(gate)) $$render(consequent);
		});
	});
	var p = $.sibling(node, 2);
	var text_1 = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text_1, $.get(loaded)), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, () => $.set(gate, !$.get(gate)));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
