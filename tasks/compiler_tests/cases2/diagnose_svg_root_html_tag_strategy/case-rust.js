import * as $ from "svelte/internal/client";
var root = $.from_svg(`<!><g><path d="M1"></path></g>`, 1);
export default function App($$anchor) {
	let raw = "<g><circle r={10}/></g>";
	var fragment = root();
	var node = $.first_child(fragment);
	$.html(node, () => raw, void 0, true);
	$.next();
	$.append($$anchor, fragment);
}
