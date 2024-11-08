import tailwindcssAnimate from "tailwindcss-animate"

/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	plugins: [tailwindcssAnimate],
	theme: {
		extend: {}
    }
}
