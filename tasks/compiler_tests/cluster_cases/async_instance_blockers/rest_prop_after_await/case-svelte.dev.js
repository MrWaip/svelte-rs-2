import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"a",
	"b",
	"c"
]);
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function inc() {
		$.update_prop(c);
	}
	var x, a, b, c, rest;
	var $$promises = $.run([async () => x = (await $.track_reactivity_loss(Promise.resolve(1)))(), () => {
		b = $.prop($$props, "b", 3, 2);
		c = $.prop($$props, "c", 15, 3);
		rest = $.rest_props($$props, rest_excludes, "rest");
	}]);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `${x ?? ""} ${$$props.a ?? ""} ${b() ?? ""} ${c() ?? ""} ${$0 ?? ""}`), [() => JSON.stringify(rest)], void 0, [$$promises[0], $$promises[1]]);
	$.delegated("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
