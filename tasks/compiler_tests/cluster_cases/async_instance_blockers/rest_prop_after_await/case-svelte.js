import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"a",
	"b",
	"c"
]);
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	function inc() {
		$.update_prop(c);
	}
	var x, a, b, c, rest;
	var $$promises = $.run([async () => x = await Promise.resolve(1), () => {
		b = $.prop($$props, "b", 3, 2);
		c = $.prop($$props, "c", 15, 3);
		rest = $.rest_props($$props, rest_excludes);
	}]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `${x ?? ""} ${$$props.a ?? ""} ${b() ?? ""} ${c() ?? ""} ${$0 ?? ""}`), [() => JSON.stringify(rest)], void 0, [$$promises[0], $$promises[1]]);
	$.delegated("click", button, inc);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
