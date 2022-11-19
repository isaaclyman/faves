module.exports = {
  purge: {
      mode: "all",
      content: [
          "./src/**/*.rs",
          "./index.html",
          "./index.scss",
          "./src/**/*.html",
          "./src/**/*.css",
      ],
  },
  theme: {
    extend: {
      colors: ({colors}) => ({
        'black': '#050505',
        'white': '#F9F9F9',
        'blue': '#0B4147',
        'lightblue': '#60D9E6',
        'bone': '#DDDBCB',
        'rust': '#CCCACE',
        'jet': '#292929'
      })
    }
  },
  variants: {},
  plugins: [],
};
