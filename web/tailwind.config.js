/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./web/src/**/*.rs"],
    theme: {
        extend: {
            colors: {
                "primary": {
                    100: "#e3d7f4",
                    200: "#c6afe9",
                    300: "#aa87de",
                    400: "#8d5fd3",
                    500: "#7137c8",
                    600: "#5a2ca0",
                    700: "#442178",
                    800: "#2d1650",
                    900: "#170b28",
                },
                "secondary": {
                    130: "#dedbe3",
                    250: "#beb7c8",
                    370: "#9d93ac",
                    500: "#7c6f91",
                    630: "#5d536c",
                    750: "#3e3748",
                    870: "#1f1c24",
                },
            },
            animation: {
                fadeIn: "fadeIn 2s",
                fadeOut: "fadeOut 2s",
            },
            keyframes: theme => ({
                fadeIn: {
                    "0%": { opacity: theme("opacity.0") },
                    "100%": { opacity: theme("opacity.100") },
                },
                fadeOut: {
                    "0%": { opacity: theme("opacity.100") },
                    "100%": { opacity: theme("opacity.0") },
                },
            }),
        },
    },
    plugins: [],
}
