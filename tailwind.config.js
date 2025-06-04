module.exports = {
    content: ["./src/**/*.{rs,html}"],
    theme: {
        extend: {
            animation: {
                'fade-in-up': 'fadeInUp 0.8s ease-out',
                'pulse-fast': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                'pulse-slow': 'pulse 8s cubic-bezier(0.4, 0, 0.6, 1) infinite',
            },
            keyframes: {
                fadeInUp: {
                    '0%': { opacity: 0, transform: 'translateY(20px)' },
                    '100%': { opacity: 1, transform: 'translateY(0)' },
                },
            },
        },
    },
    plugins: [],
};