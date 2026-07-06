import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Repo {
			tree;
			async #tree() {}
		}
		const repo = new Repo();
	});
}
