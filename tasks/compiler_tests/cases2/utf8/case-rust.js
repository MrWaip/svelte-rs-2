import * as $ from "svelte/internal/client";
var root = $.template(`<div>🌞👨‍💻</div><div>ютф кейс</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.append($$anchor, fragment);
}
