/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./web/src/**/*.rs"],
    theme: {
        colors: {
            transparent: "transparent",
            current: "currentColor",
            "black": "#000000",
            "white": "#ffffff",
            "gray": {
                130: "#dedbe3ff",
                250: "#beb7c8ff",
                370: "#9d93acff",
                500: "#7c6f91ff",
                630: "#5d536cff",
                750: "#3e3748ff",
                870: "#1f1c24ff",
            },
            "violet": {
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
        },
        extend: {},
    },
    plugins: [],
}
