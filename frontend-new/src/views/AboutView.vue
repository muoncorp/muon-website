<template>
  <div>
    <ImageHeader
      :src="require('../assets/about.jpg')"
      title="About"
    />
    <v-container>
      <h2
        id="vision"
        class="pb-2"
      >
        Vision
      </h2>
      <p>
        현 시대는 4차 산업혁명을 맞이하고 있습니다.
      </p>
      <p>
        빅데이터, 인공지능(AI), 사물인터넷(IoT) 등은 거대한 변화를 일으키고 있습니다.
      </p>
      <p>
        뮤온은 뛰어난 기술력을 가진 인재들이 모인 IoT 전문 솔루션 회사로, 미래생활을 편리하게 발전시키고자 다양한 시도를 하고 있습니다.
      </p>
      <p>
        창의적인 개발과 도전을 추구하며 최신 기술 동향을 지속적으로 연구, 도입하여 우수한 제품을 제공합니다.
      </p>
      <p>
        고객과의 긴밀한 협력을 통해 요구사항을 빠르게 충족시키고 높은 수준의 기술력을 바탕으로 고객의 성공에 부합하는 최상의 임베디드 솔루션을 제공할 것입니다.
      </p>
      <p>
        감사합니다.
      </p>
      <p class="mt-2 font-weight-bold">
        뮤온 주식회사 대표이사 박 흥 준
      </p>
      <h2 id="history">
        History
      </h2>
      <v-timeline
        align-top
        dense
      >
      <v-timeline-item
          color="pink"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2023.08</strong>
            </v-col>
            <v-col>
              <strong>M사 시각 장애인을 위한 점자 학습기 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="teal lighten-3"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2023.06</strong>
            </v-col>
            <v-col>
              <strong>고용노동부 일·생활 균형 캠페인 참여</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="pink"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2022.12</strong>
            </v-col>
            <v-col>
              <strong>G사 보안키 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="teal lighten-3"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2022.09</strong>
            </v-col>
            <v-col>
              <strong>L사 교육용 자율주행차 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="pink"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2022.05</strong>
            </v-col>
            <v-col>
              <strong>C사 AI-Gage 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="teal lighten-3"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2022.03</strong>
            </v-col>
            <v-col>
              <strong>MUON-SBC-A40i 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="pink"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2021.09</strong>
            </v-col>
            <v-col>
              <strong>본사 확장 이전</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="teal lighten-3"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2020.10</strong>
            </v-col>
            <v-col>
              <strong>산학협력 체결 (유한공업고등학교)</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="pink"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2020.08</strong>
            </v-col>
            <v-col>
              <strong>MUON-IOT Digital Signage 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="teal lighten-3"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2020.05</strong>
            </v-col>
            <v-col>
              <strong>MUON-SBC-A64 개발</strong>
            </v-col>
          </v-row>
        </v-timeline-item>

        <v-timeline-item
          color="pink"
          small
        >
          <v-row class="pt-1">
            <v-col cols="3">
              <strong>2019.10</strong>
            </v-col>
            <v-col>
              <strong>뮤온(주) 법인 설립</strong>
            </v-col>
          </v-row>
        </v-timeline-item>
      </v-timeline>
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
  import { Component, Vue } from 'vue-property-decorator'
  import ImageHeader from '@/components/ImageHeader.vue'

  @Component({
    components: {
      ImageHeader,
    },
  })
  export default class AboutView extends Vue {
    fab = false

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
</style>
