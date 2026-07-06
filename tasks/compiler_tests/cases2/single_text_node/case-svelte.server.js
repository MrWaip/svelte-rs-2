import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!---->some long text line`);
}
