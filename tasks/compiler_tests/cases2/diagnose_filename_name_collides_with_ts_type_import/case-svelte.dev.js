DepositMethod[$.FILENAME] = "DepositMethod.svelte";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.add_locations($.from_html(`<p> </p>`), DepositMethod[$.FILENAME], [[6, 0]]);
export default function DepositMethod($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, DepositMethod);
	let props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $$props.title));
	$.append($$anchor, p);
	return $.pop($$exports);
}
