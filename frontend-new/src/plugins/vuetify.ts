import 'material-design-icons-iconfont/dist/material-design-icons.css'
import Vue from 'vue';
import Vuetify from 'vuetify/lib/framework';

Vue.use(Vuetify);

export default new Vuetify({
  icons: {
    iconfont: 'md',
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
});
