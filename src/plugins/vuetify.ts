import Vue from 'vue'
import Vuetify from 'vuetify/lib'

Vue.use(Vuetify)

export default new Vuetify({
  icons: {
    iconfont: 'mdi',
  },
  theme: {
    dark: false,
    themes: {
      light: {
        primary: '#022c4c',
        secondary: '#fdfdfd',
        accent: '#b5c3d9',
        success: '#ffd012',
      },
    },
  },
})
