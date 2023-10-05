<template>
  <div>
    <ImageHeader
      :src="require('../assets/products.jpg')"
      title="Products"
    />
    <v-container>
      <h2 id="touch">
        Smart Touch Controller
      </h2>
      <v-row>
        <v-col>
          <v-carousel class="m-carousel">
            <v-carousel-item
              src="../assets/touch_1.jpg"
              reverse-transition="fade-transition"
              transition="fade-transition"
            />
            <v-carousel-item
              src="../assets/touch_2.jpg"
              reverse-transition="fade-transition"
              transition="fade-transition"
            />
          </v-carousel>
        </v-col>
        <v-col>
          <ul>
            <li>
              Quad Core Cortex-A53 64bit SoC
            </li>
            <li>
              DDR3L 1GB, EMMC 16GB
            </li>
            <li>
              7 inch LCD + CTP (Resolution: 800x480)
            </li>
            <li>
              4.3 inch LCD + CTP (Resolution: 480x272)
            </li>
            <li>
              OS: Android 7.0
            </li>
          </ul>
        </v-col>
      </v-row>
      <h2 id="io-board" class="my-2">
        Arcade Game Machine IO board
      </h2>
      <v-row>
        <v-row>
          <v-col>
            <ul>
              <li>
                STM32F103 MCU 기반의 IO Board
              </li>
              <li>
                IO: Input 16EA / Output 16EA (Open Drain) / Solenoid Meter 8EA
              </li>
              <li>
                Lithium Battery PCM, Charger 및 Load Share 회로 적용
              </li>
              <li>
                Class D Amp 회로 적용
              </li>
              <li>
                I2C interface EEPROM 적용 및 Log 기록
              </li>
            </ul>
          </v-col>
          <v-col>
            <v-carousel class="m-carousel">
              <v-carousel-item
                src="../assets/io_1.jpg"
                reverse-transition="fade-transition"
                transition="fade-transition"
              />
              <v-carousel-item
                src="../assets/io_2.jpg"
                reverse-transition="fade-transition"
                transition="fade-transition"
              />
            </v-carousel>
          </v-col>
        </v-row>
      </v-row>
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
    </v-container>
  </div>
</template>

<script lang="ts">
  import { Component, Vue, Watch } from 'vue-property-decorator'
  import ImageHeader from '@/components/ImageHeader.vue'

  @Component({
    components: {
      ImageHeader,
    },
  })
  export default class ProductsView extends Vue {
    fab = false
    private vrIimageSrcs= [
      '../assets/vr_1.jpg',
      '../assets/vr_2.jpg',
      '../assets/vr_3.jpg',
    ]

    mounted () {
      this.routeUpdated()
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
.m-carousel {
  width: 100%;
  max-width: 500px;
  max-height: 400px;
}
</style>
