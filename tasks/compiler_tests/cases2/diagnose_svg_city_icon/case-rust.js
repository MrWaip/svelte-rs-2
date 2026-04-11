import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg viewBox="0 0 24 24" fill="none" width="24" height="24" xmlns="http://www.w3.org/2000/svg"><rect x="3" y="11" width="6" height="10" stroke="#0070f3" stroke-width="2"></rect><rect x="9" y="7" width="6" height="14" stroke="#0070f3" stroke-width="2"></rect><rect x="15" y="3" width="6" height="18" stroke="#0070f3" stroke-width="2"></rect></svg>`);
export default function App($$anchor) {
	var svg = root();
	$.next(2);
	$.reset(svg);
	$.append($$anchor, svg);
	// City icon
}
