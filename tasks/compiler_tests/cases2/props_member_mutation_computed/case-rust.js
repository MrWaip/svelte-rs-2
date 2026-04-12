App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let user = $.prop($$props, "user", 23, () => ({ profile: {} }));
	let key = "name";
	function rename() {
		$$ownership_validator.mutation("user", [
			"user",
			"profile",
			key
		], user().profile[key] = "next", 6, 2);
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
