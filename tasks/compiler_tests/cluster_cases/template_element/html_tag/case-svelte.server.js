import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<template id="t3">1${$.html("<b>B</b>")}1</template>`);
}
