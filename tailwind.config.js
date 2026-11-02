/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./sso.html", "./web/**/*.{vue,js,ts,jsx,tsx}"],
    theme: {
        extend: {
            height: {
                "80vh": "80vh",
                "40vh": "40vh",
            },
            backgroundImage: {
                // Shared page background, used by every full-page view.
                "page-surface":
                    "radial-gradient(circle at 50% 0%, rgba(56, 189, 248, 0.14), transparent 36rem)," +
                    " radial-gradient(circle at 15% 0%, rgba(34, 211, 238, 0.1), transparent 32rem)," +
                    " linear-gradient(135deg, #0f2731 0%, #183e49 48%, #112e38 100%)",
            },
        },
    },
    plugins: [],
};
