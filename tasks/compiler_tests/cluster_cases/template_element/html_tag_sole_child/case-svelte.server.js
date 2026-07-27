import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { s } = $$props;
	$$renderer.push(`<template>${$.html(s)}</template>`);
}
