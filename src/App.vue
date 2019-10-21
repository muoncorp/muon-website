<template>
  <v-app>
    <v-app-bar app>
      <v-toolbar-title class="headline text-uppercase">
        <span>뮤온</span>
        <span class="font-weight-light">MUON</span>
      </v-toolbar-title>
      <v-spacer />
      <v-btn
        text
        href="https://github.com/vuetifyjs/vuetify/releases/latest"
        target="_blank"
      >
        <span class="mr-2">Latest Release</span>
      </v-btn>
    </v-app-bar>

    <v-content>
      <hello-world />
    </v-content>

    <v-footer
      color="primary lighten-1"
      app
    >
      <v-row justify="left">
        <div>
          뮤온(주) | 대표이사: 박홍석
        </div>
        <div>
          서울시 영등포구 양평로21가길 19 선유도우림라이온스밸리 B동 509호
        </div>
        <div>
          사업자등록번호: xxx-xx-xxxxx
        </div>
      </v-row>
      <v-row justify="center">
        &copy; Copyright {{ new Date().getFullYear() }}, MUON Corp. All Rights
        Reserved
      </v-row>
    </v-footer>
  </v-app>
</template>

<script lang="ts">
  import Vue from 'vue'
  import HelloWorld from './components/HelloWorld.vue'

  export default Vue.extend({
    name: 'App',
    components: {
      HelloWorld,
    },
    data: () => ({
      //
    }),
  })
</script>
