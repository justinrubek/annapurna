const colors = require('tailwindcss/colors')

const themes = {
    hypno: {
        colors: {
            text: colors.gray[300],
            primary: colors.purple[700],
            secondary: colors.purple[500],
            dark: {
                primary: colors.purple[400],
                secondary: colors.purple[900],
            },
            light: {
                neutral: colors.white[200],
            },
            accent: {
                gray: {
                    light: colors.blue[300],
                    dark: colors.blue[500]
                },
                default: colors.blue[700]
            }
        }
    },
}

module.exports = {
    ...themes,
    default: themes.hypno,
}
