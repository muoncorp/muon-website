<template>
  <div>
    <ImageHeader
      :src="require('../assets/jobs.jpg')"
      title="Jobs"
    />
    <v-container>
      <h2>
        채용 절차
      </h2>
      <v-timeline>
        <v-timeline-item
          v-for="(item, i) in recruitProcess"
          :key="i"
          :color="item.color"
          small
        >
          <template v-slot:opposite>
            <h3>
              {{ item.title }}
            </h3>
          </template>
          <v-card
            v-if="item.text.length > 0"
            class="pa-2 jobs-card"
          >
            <div
              v-for="(line, j) in item.text"
              :key="j"
            >
              {{ line }}
            </div>
          </v-card>
        </v-timeline-item>
      </v-timeline>
      <h3>
        입사 지원 및 채용 문의
      </h3>
      <v-icon class="mr-1">
        mail
      </v-icon>
      <a href="mailto: recruit@muon.co">recruit@muon.co</a>
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
  export default class JobsView extends Vue {
    recruitProcess = [
      {
        title: '1. 서류 전형',
        text: [
          '신입: 이력서, 자기 소개서',
          '경력: 이력서, 경력 기술서',
        ],
        color: 'cyan',
      },
      {
        title: '2. 실무 면접',
        text: [
          '신입: 필기/실기 시험, 실무자 면접',
          '경력: 포트폴리오 프리젠테이션, 실무자 면접',
        ],
        color: 'green',
      },
      {
        title: '3. 임원 면접',
        text: [
          '임원 면접 및 채용 조건 협의',
        ],
        color: 'pink',
      },
      {
        title: '4. 채용 확정',
        text: [],
        color: 'orange',
      },
    ]
  }
</script>

<style lang="scss" scoped>
.jobs-card {
  min-height: 80px;
}
</style>
