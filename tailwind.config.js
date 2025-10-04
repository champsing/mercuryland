/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./sso.html", "./web/**/*.{vue,js,ts,jsx,tsx}"],
    theme: {
        extend: {
            height: {
                "80vh": "80vh",
                "40vh": "40vh",
            },
        },
    },
    plugins: [],
};
