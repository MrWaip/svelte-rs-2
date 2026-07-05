App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { scale } from "./utils.js";
var root = $.add_locations($.from_svg(`<polyline></polyline>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = $.tag_proxy($.proxy([
		10,
		20,
		30
	]), "data");
	const x = $.tag($.derived(() => scale([0, data.length], [0, 100])), "x");
	const y = $.tag($.derived(() => scale([0, 30], [100, 0])), "y");
	var $$exports = { ...$.legacy_api() };
	var polyline = root();
	$.template_effect(($0) => $.set_attribute(polyline, "points", $0), [() => data.map((d, i) => [$.get(x)(i), $.get(y)(d)]).join(" ")]);
	$.append($$anchor, polyline);
	return $.pop($$exports);
}
