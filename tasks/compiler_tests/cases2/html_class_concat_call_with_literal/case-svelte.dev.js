App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { getProductName } from "./helpers";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(($0) => $.set_class(div, 1, `x0${$0 ?? ""}`), [() => getProductName()]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
