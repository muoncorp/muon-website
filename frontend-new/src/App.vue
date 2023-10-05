<template>
  <v-app>
    <v-app-bar
      app
      flat
      dense
      color="primary"
    >
      <v-container>
        <v-row>
          <router-link to="/">
            <v-img
              class="align-self-end my-1"
              src="./assets/logo.png"
              max-width="70"
            />
          </router-link>
          <v-spacer />
          <div
            v-if="$vuetify.breakpoint.smAndDown"
            :style="{ 'cursor': 'pointer' }"
            @click.stop="drawer = !drawer"
          >
            <span
              v-if="currentItem"
              class="white--text"
            >
              {{ currentItem.text }}
            </span>
            <v-app-bar-nav-icon
              color="white"
            />
          </div>
          <div v-else>
            <MenuButton
              v-for="(item, index) in menuItems"
              :key="index"
              :to="item.to"
              :text="item.text"
              :items="item.items"
            />
          </div>
        </v-row>
      </v-container>
    </v-app-bar>

    <v-main>
      <router-view />
      <v-fab-transition>
        <v-btn
          v-show="fab"
          v-scroll="onScroll"
          class="mb-10"
          color="primary lighten-3"
          fixed
          bottom
          right
          fab
          @click="toTop"
        >
          <v-icon>
            mdi-chevron-up
          </v-icon>
        </v-btn>
      </v-fab-transition>
      <v-navigation-drawer
        v-model="drawer"
        fixed
        temprory
        right
        class="pa-2"
        width="180px"
        height="460px"
        :style="{'z-index': 100, 'top': '48px'}"
      >
        <div
          v-for="(item, i) in menuItems"
          :key="i"
        >
          <v-btn
            text
            :to="item.to"
            class="mb-1"
            :style="{ 'width': '100%', 'justify-content': 'left' }"
          >
            <span :style="{ 'text-transform': 'none' }">
              {{ item.text }}
            </span>
          </v-btn>
          <div v-if="item.items">
            <div
              v-for="(subItem, j) in item.items"
              :key="j"
              class="ml-5 pl-1 pb-1"
              :style="{ 'border-left': '1px solid black' }"
            >
              <v-btn
                text
                :to="subItem.to"
                :style="{ 'width': '100%', 'justify-content': 'left', 'text-transform': 'none' }"
              >
                <span>
                  {{ subItem.text }}
                </span>
              </v-btn>
            </div>
          </div>
        </div>
      </v-navigation-drawer>
    </v-main>

    <v-footer
      class="d-flex flex-column"
      app
      absolute
    >
      <v-row
        class="d-flex flex-column justify-end subtitle-2 my-1"
      >
        <div>
          뮤온(주) | 대표이사: 박흥준
        </div>
        <div
          class="d-flex flex-row flex-wrap"
        >
          <div class="mr-1">
            서울시 영등포구 양평로21가길 19,
          </div>
          <div>
            선유도우림라이온스밸리 B동 304호
          </div>
        </div>
        <div
          class="d-flex flex-row flex-wrap"
        >
          <div class="mr-4">
            사업자등록번호: 886-86-01576
          </div>
          <div>
            TEL: 02)2635-6789
          </div>
        </div>
        <div
          class="d-flex flex-row flex-wrap"
        >
          <div class="mr-2">
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
  import { Component, Vue, Watch } from 'vue-property-decorator'
  import MenuButton from './components/MenuButton.vue'
  import { Item } from './item'

   @Component({
    components: {
      MenuButton,
    },
  })
  export default class App extends Vue {
    drawer = false
    fab = false
    appBarHeight = '0px'
    menuItems: Item[] = [
      {
        to: '/',
        text: 'Home',
      },
      {
        to: '/about',
        text: 'About',
        items: [
          {
            to: '/about#vision',
            text: 'Vision',
          },
          {
            to: '/about#history',
            text: 'History',
          },
        ],
      },
      {
        to: '/products',
        text: 'Products',
      },
      // {
      //   to: '/blog',
      //   text: 'Blog',
      // },
      {
        to: '/jobs',
        text: 'Jobs',
      },
      {
        to: '/contact',
        text: 'Contact',
        items: [
          {
            to: '/contact#contact',
            text: 'Contact Us',
          },
          {
            to: '/contact#location',
            text: 'Location',
          },
        ],
      },
    ]

    get currentItem (): Item|null {
      for (const item of this.menuItems) {
        if (item.items) {
          for (const subItem of item.items) {
            if (subItem.to === this.$route.path) {
              return subItem
            }
          }
        }
        if (item.to === this.$route.path) {
          return item
        }
      }
      return null
    }

    @Watch('$route')
    routeUpdated () {
      if (this.$route.hash) {
        this.$vuetify.goTo(this.$route.hash)
      }
    }

    onScroll (e: any) {
      if (typeof window === 'undefined') {
        return
      }
      const top = window.pageYOffset || e.target.scrollTop || 0
      this.fab = top > 20
    }

    toTop () {
      this.$vuetify.goTo(0)
    }
  }
</script>

<style lang="scss" scoped>
@import url('https://fonts.googleapis.com/css?family=Noto+Sans+KR:100,300,400,500,700,900');
@import url('https://cdn.jsdelivr.net/npm/@mdi/font@latest/css/materialdesignicons.min.css');

$body-font-family: 'Noto Sans KR';

.v-application {
  font-family: $body-font-family, sans-serif !important;
}
</style>
