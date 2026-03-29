import * as $ from "svelte/internal/client";
import { scale } from "./utils.js";
var root = $.from_svg(`<polyline></polyline>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let data = $.proxy([
		10,
		20,
		30
	]);
	const x = $.derived(() => scale([0, data.length], [0, 100]));
	const y = $.derived(() => scale([0, 30], [100, 0]));
	var polyline = root();
	$.template_effect(($0) => $.set_attribute(polyline, "points", $0), [() => data.map((d, i) => [$.get(x)(i), $.get(y)(d)]).join(" ")]);
	$.append($$anchor, polyline);
	$.pop();
}
