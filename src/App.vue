<template>
  <v-app>
    <v-app-bar
      app
      color="primary"
    >
      <v-toolbar-title class="headline fill-height">
        <router-link to="/">
          <v-img
            alt="MUON logo"
            src="./assets/muon-logo.png"
            width="88.88px"
            height="40.53px"
            class="mt-1"
          />
        </router-link>
        <!-- <span class="ml-2">뮤온</span>
        <span class="font-weight-light"> 주식회사</span> -->
      </v-toolbar-title>
      <v-spacer />
      <v-menu
        open-on-hover
        bottom
        offset-y
      >
        <template v-slot:activator="{ on }">
          <v-btn
            text
            to="/about"
            v-on="on"
          >
            <span class="text-uppercase">about</span>
          </v-btn>
        </template>
        <v-list>
          <v-list-item
            v-for="(item, n) in aboutItems"
            :key="n"
            :to="item.to"
          >
            <v-list-item-title>
              {{ item.text }}
            </v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
      <v-btn
        text
        dark
        to="/home"
      >
        <span class="text-uppercase">jobs</span>
      </v-btn>
      <v-btn
        text
        to="/contact"
      >
        <span class="text-uppercase">contact</span>
      </v-btn>
    </v-app-bar>

    <v-content>
      <router-view />
    </v-content>

    <v-footer
      class="d-flex flex-column"
      app
      color="primary"
    >
      <v-row
        class="d-flex flex-column justify-start"
      >
        <div>
          뮤온(주) | 대표이사: 박홍석
        </div>
        <div>
          TEL: 02)2635-6789
        </div>
        <div>
          서울시 영등포구 양평로21가길 19, 선유도우림라이온스밸리 B동 509호
        </div>
        <div>
          사업자등록번호: 886-86-01576
        </div>
      </v-row>
      <v-row justify="center">
        &copy; Copyright {{ new Date().getFullYear() }}, MUON Corp. All Rights
        Reserved.
      </v-row>
    </v-footer>
  </v-app>
</template>

<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator'
  import HelloWorld from './components/HelloWorld.vue'

  @Component({
    components: {
      HelloWorld,
    },
  })
  export default class App extends Vue {
    private aboutItems = [{
      text: 'overview',
      to: '/about#overview',
    }, {
      text: 'vision',
      to: '/about#vision',
    }, {
      text: 'history',
      to: '/about#history',
    }];
  }
</script>

<style lang="scss">
</style>
