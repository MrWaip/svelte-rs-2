import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const greet = () => "hi";
	var $$exports = {
		...$.legacy_api(),
		get greet() {
			return greet;
		}
	};
	$.bind_prop($$props, "greet", greet);
	return $.pop($$exports);
}
