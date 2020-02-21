<template>
  <v-app>
    <v-app-bar
      app
      hide-on-scroll
      extended
      fade-img-on-scroll
      color="primary"
    >
      <v-toolbar-title class="headline fill-height">
        <!-- <v-img
          alt="MUON logo"
          src="./assets/muon-logo.png"
          width="88.88px"
          height="40.53px"
          class="mt-1"
        /> -->
      </v-toolbar-title>
      <v-spacer />
      <template v-slot:img>
        <v-img
          src="./assets/header.jpg"
        />
      </template>
      <template v-slot:extension>
        <router-link to="/">
          <v-img
            class="align-self-end mb-1"
            src="./assets/logo.png"
            max-width="78"
          />
        </router-link>
        <v-spacer />
        <MenuButton
          v-for="(item, index) in menuItems"
          :key="index"
          :to="item.to"
          :text="item.text"
          :items="item.items"
        />
      </template>
    </v-app-bar>

    <v-content>
      <router-view />
    </v-content>

    <v-footer
      class="d-flex flex-column"
      app
      absolute
    >
      <v-row
        class="d-flex flex-column justify-end subtitle-2"
      >
        <div>
          뮤온(주) | TEL: 02)2635-6789
        </div>
        <div
          class="d-flex flex-row flex-wrap"
        >
          <div>
            서울시 영등포구 양평로21가길 19,
          </div>
          <div>
            선유도우림라이온스밸리 B동 509호
          </div>
        </div>
        <div
          class="d-flex flex-row flex-wrap"
        >
          <div>
            &copy; Copyright {{ new Date().getFullYear() }}, MUON Corp.
          </div>
          <div>
            All Rights Reserved.
          </div>
        </div>
      </v-row>
    </v-footer>
  </v-app>
</template>

<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator'
  import HelloWorld from './components/HelloWorld.vue'
  import MenuButton from './components/MenuButton.vue'

  @Component({
    components: {
      HelloWorld,
      MenuButton,
    },
  })
  export default class App extends Vue {
    private menuItems = [{
      to: '/about',
      text: 'about',
      items: [{
        to: '/about#overview',
        text: 'Overview',
      }, {
        to: '/about#vision',
        text: 'Vision',
      }, {
        to: '/about#history',
        text: 'History',
      }],
    }, {
      to: '/jobs',
      text: 'jobs',
    }, {
      to: '/contact',
      text: 'contact',
      items: [{
        to: '/contact#contact',
        text: 'Contact Us',
      }, {
        to: '/contact#location',
        text: 'Location',
      }],
    }]
  }
</script>

<style lang="scss" scoped>
</style>
