import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { user = { profile: {} } } = $$props;
		let key = "name";
		function rename() {
			user.profile[key] = "next";
		}
	});
}
